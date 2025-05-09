use frame_support::pallet_macros::pallet_section;

/// Define the implementation of the pallet, like helper functions.
#[pallet_section]
mod impls {
    use sp_std::mem;
    use sp_io::hashing;
    use frame_support::sp_runtime::offchain::{http, Duration};
    use serde_json_core;
    use core::{str};

    #[derive(Deserialize)]
    struct Quota {
        USD: f64,
    }
            
    impl<T: Config> Pallet<T> {
        // get a random 256.
        pub fn random_value(who: &T::AccountId) -> [u8; 16] {
            let nonce = frame_system::Pallet::<T>::account_nonce(&who);
            // let nonce_u32: u32 = nonce as u32;
            // generate a random value based on account and its nonce
            let nonce_u32: u32 = TryInto::try_into(nonce).ok().expect("nonce is u64; qed");
            let bytes = unsafe { mem::transmute::<u32, [u8; 4]>(nonce_u32) };

            // payload.using_encoded(blake2_128)
            hashing::blake2_128(&bytes)
        }

        // breed on kitty based on both paraent kitties
        fn breed_kitty(who: &T::AccountId, kitty_1: [u8; 16], kitty_2: [u8; 16]) -> [u8; 16] {
            let selector = Self::random_value(&who);

            let mut data = [0u8; 16];
            for i in 0..kitty_1.len() {
                // 0 choose kitty2, and 1 choose kitty1
                data[i] = (kitty_1[i] & selector[i]) | (kitty_2[i] & !selector[i]);
            }
            data
        }

        fn mint_kitty(data: [u8; 16], owner: &T::AccountId) -> DispatchResult {
            T::Currency::reserve(&owner, T::KittyCost::get()).map_err(|_| Error::<T>::BalanceNotEnough)?;

            let kittie = Kitty { gene: data, price: 0 };
            let id = NextKittyId::<T>::get();
            Kitties::<T>::insert(id, kittie);
            NextKittyId::<T>::set(id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?);
            KittyOwner::<T>::insert(id, owner);

            Self::deposit_event(Event::KittyCreated {
                creator: owner.clone(),
                index: id,
                data: data,
            });

            Ok(())
        }

        fn transfer_kitty(from: T::AccountId, to: T::AccountId, kitty_id: u32) -> DispatchResult {
            T::Currency::reserve(&to, T::KittyCost::get()).map_err(|_| Error::<T>::BalanceNotEnough)?;
            T::Currency::unreserve(&from, T::KittyCost::get());

            KittyOwner::<T>::set(kitty_id, Some(to.clone()));

            Self::deposit_event(Event::KittyTransferred {
                from: from,
                to: to,
                index: kitty_id,
            });

            Ok(())
        }

        fn sell_kitty(kitty_id: u32, price: BalanceOf<T>, until_block: BlockNumberFor<T>) -> DispatchResult {
            KittiesBid::<T>::insert(kitty_id, BoundedVec::new());
            KittiesSaleInfo::<T>::insert(kitty_id, (price, until_block));

            Self::deposit_event(Event::KittyOnSale {
                index: kitty_id,
                price,
                until_block,
            });

            Ok(())
        }

        fn bid_for_kitty(who: T::AccountId, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
            let mut v = KittiesBid::<T>::get(kitty_id).unwrap();
            if v.last().is_some() {
                ensure!(price >= v.last().unwrap().1 + T::BidMargin::get(), Error::<T>::BidPriceTooLow);
            } else {
                ensure!(price >= KittiesSaleInfo::<T>::get(kitty_id).unwrap().0, Error::<T>::BidPriceTooLow);
            }

            T::Currency::reserve(&who, price).map_err(|_| Error::<T>::BalanceNotEnough)?;

            v.try_push((who, price)).map_err(|_| Error::<T>::NewBidError)?;
            KittiesBid::<T>::set(kitty_id, Some(v));

            Ok(())
        }

        /// Fetch current price and return the result in 0.01 cents.
        fn fetch_price() -> Result<u64, http::Error> {
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
            let body_str = str::from_utf8(&body).map_err(|_| {
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

            log::warn!("Got price: {} 0.01 cents", price);

            Ok(price)
        }

        /// Parse the price from the given JSON string using `lite-json`.
        ///
        /// Returns `None` when parsing failed or `Some(price in cents)` when parsing is successful.
        fn parse_price(price_str: &str) -> Option<u64> {
            let t: serde_json_core::de::Result<(Quota, usize)> = serde_json_core::from_str(price_str);

            if t.is_ok() {
                let quota = t.unwrap();
                if quota.1 <= 0 {
                    return None;
                } 

                let exchange = quota.0.USD;
                return Some((exchange * 10000.0) as u64); 
            }            
            
            return None;
        }        
    }
}
