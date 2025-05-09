#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_macros::pallet_section;

#[pallet_section]
mod crypto {
    extern crate alloc;
    use alloc::vec::Vec;
    use frame_system::offchain::{
        CreateSignedTransaction, SendSignedTransaction, SignedPayload, Signer, SigningTypes,
        SubmitTransaction,
    };
    use lite_json::json::JsonValue;
    use sp_core::crypto::KeyTypeId;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        offchain::{
            http,
            storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
            Duration,
        },
        traits::Verify,
        MultiSignature, MultiSigner,
    };

    pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"kity");
    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;

    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    // implemented for mock runtime in test
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for TestAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    impl<T: Config> Pallet<T> {
        fn decide_to_send(block_number: BlockNumberFor<T>) -> bool {
            /// A friendlier name for the error that is going to be returned in case we are in the grace
            /// period.
            const RECENTLY_SENT: () = ();

            // Start off by creating a reference to Local Storage value.
            // Since the local storage is common for all offchain workers, it's a good practice
            // to prepend your entry with the module name.
            let val = StorageValueRef::persistent(b"kitties::last_send");
            // The Local Storage is persisted and shared between runs of the offchain workers,
            // and offchain workers may run concurrently. We can use the `mutate` function, to
            // write a storage entry in an atomic fashion. Under the hood it uses `compare_and_set`
            // low-level method of local storage API, which means that only one worker
            // will be able to "acquire a lock" and send a transaction if multiple workers
            // happen to be executed concurrently.
            let res = val.mutate(
                |last_send: Result<Option<BlockNumberFor<T>>, StorageRetrievalError>| {
                    match last_send {
                        // If we already have a value in storage and the block number is recent enough
                        // we avoid sending another transaction at this time.
                        Ok(Some(block)) if block_number < block + T::GracePeriod::get() => {
                            Err(RECENTLY_SENT)
                        }
                        // In every other case we attempt to acquire the lock and send a transaction.
                        _ => Ok(block_number),
                    }
                },
            );
            match res {
                // The value has been set correctly, which means we can safely send a transaction now.
                Ok(_block_number) => true,
                // We are in the grace period, we should not send a transaction this time.
                Err(MutateStorageError::ValueFunctionFailed(RECENTLY_SENT)) => false,
                // We wanted to send a transaction, but failed to write the block number (acquire a
                // lock). This indicates that another offchain worker that was running concurrently
                // most likely executed the same logic and succeeded at writing to storage.
                // Thus we don't really want to send the transaction, knowing that the other run
                // already did.
                Err(MutateStorageError::ConcurrentModification(_)) => false,
            }
        }

        pub(crate) fn fetch_price_and_send_signed() -> Result<(), &'static str> {
            let signer = Signer::<T, T::AuthorityId>::all_accounts();
            if !signer.can_sign() {
                return Err(
                    "No local accounts available. Consider adding one via `author_insertKey` RPC.",
                );
            }
            // Make an external HTTP request to fetch the current price.
            // Note this call will block until response is received.
            let price = Self::fetch_price().map_err(|_| "Failed to fetch price")?;

            // Using `send_signed_transaction` associated type we create and submit a transaction
            // representing the call, we've just created.
            // Submit signed will return a vector of results for all accounts that were found in the
            // local keystore with expected `KEY_TYPE`.
            let results = signer.send_signed_transaction(|_account| {
                // Received price is wrapped into a call to `submit_price` public function of this
                // pallet. This means that the transaction, when executed, will simply call that
                // function passing `price` as an argument.
                Call::submit_price { price }
            });

            for (acc, res) in &results {
                match res {
                    Ok(()) => log::info!("[{:?}] Submitted price of {} cents", acc.id, price),
                    Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
                }
            }

            Ok(())
        }

        /// Fetch current price and return the result in cents.
        pub(crate) fn fetch_price() -> Result<u32, http::Error> {
            // We want to keep the offchain worker execution time reasonable, so we set a hard-coded
            // deadline to 2s to complete the external call.
            // You can also wait indefinitely for the response, however you may still get a timeout
            // coming from the host machine.
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
            // Initiate an external HTTP GET request.
            // This is using high-level wrappers from `sp_runtime`, for the low-level calls that
            // you can find in `sp_io`. The API is trying to be similar to `request`, but
            // since we are running in a custom WASM execution environment we can't simply
            // import the library here.
            let request = http::Request::get(
                "https://min-api.cryptocompare.com/data/price?fsym=DOT&tsyms=USD",
            );
            // We set the deadline for sending of the request, note that awaiting response can
            // have a separate deadline. Next we send the request, before that it's also possible
            // to alter request headers or stream body content in case of non-GET requests.
            let pending = request
                .deadline(deadline)
                .send()
                .map_err(|_| http::Error::IoError)?;

            // The request is already being processed by the host, we are free to do anything
            // else in the worker (we can send multiple concurrent requests too).
            // At some point however we probably want to check the response though,
            // so we can block current thread and wait for it to finish.
            // Note that since the request is being driven by the host, we don't have to wait
            // for the request to have it complete, we will just not read the response.
            let response = pending
                .try_wait(deadline)
                .map_err(|_| http::Error::DeadlineReached)??;
            // Let's check the status code before we proceed to reading the response.
            if response.code != 200 {
                log::warn!("Unexpected status code: {}", response.code);
                return Err(http::Error::Unknown);
            }

            // Next we want to fully read the response body and collect it to a vector of bytes.
            // Note that the return object allows you to read the body in chunks as well
            // with a way to control the deadline.
            let body = response.body().collect::<Vec<u8>>();

            // Create a str slice from the body.
            let body_str = alloc::str::from_utf8(&body).map_err(|_| {
                log::warn!("No UTF8 body");
                http::Error::Unknown
            })?;

            let price = match Self::parse_price(body_str) {
                Some(price) => Ok(price),
                None => {
                    log::warn!("Unable to extract price from the response: {:?}", body_str);
                    Err(http::Error::Unknown)
                }
            }?;

            log::warn!("Got price: {} cents", price);

            Ok(price)
        }

        /// Parse the price from the given JSON string using `lite-json`.
        ///
        /// Returns `None` when parsing failed or `Some(price in cents)` when parsing is successful.
        fn parse_price(price_str: &str) -> Option<u32> {
            let val = lite_json::parse_json(price_str);
            let price = match val.ok()? {
                JsonValue::Object(obj) => {
                    let (_, v) = obj
                        .into_iter()
                        .find(|(k, _)| k.iter().copied().eq("USD".chars()))?;
                    match v {
                        JsonValue::Number(number) => number,
                        _ => return None,
                    }
                }
                _ => return None,
            };

            let exp = price.fraction_length.saturating_sub(2);
            Some(price.integer as u32 * 100 + (price.fraction / 10_u64.pow(exp)) as u32)
        }

        /// Add new price to the list.
        fn add_price(maybe_who: Option<T::AccountId>, price: u32) {
            log::info!("Adding to the average: {}", price);
            <Prices<T>>::mutate(|prices| {
                if prices.try_push(price).is_err() {
                    prices[(price % T::MaxPrices::get()) as usize] = price;
                }
            });

            let average = Self::average_price()
                .expect("The average is not empty, because it was just mutated; qed");
            log::info!("Current average price is: {}", average);
            // here we are raising the NewPrice event
            Self::deposit_event(Event::NewPrice { price, maybe_who });
        }

        /// Calculate current average price.
        pub(crate) fn average_price() -> Option<u32> {
            let prices = Prices::<T>::get();
            if prices.is_empty() {
                None
            } else {
                Some(prices.iter().fold(0_u32, |a, b| a.saturating_add(*b)) / prices.len() as u32)
            }
        }
    }
}
