use frame_support::pallet_macros::pallet_section;
use frame_system::{
    offchain::{
        AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
        SignedPayload, Signer, SigningTypes, SubmitTransaction,
    },
    pallet_prelude::BlockNumberFor,
};
/// Define the implementation of the pallet, like helper functions.
#[pallet_section]
mod impls {
    impl<T: Config> Pallet<T> {
        // get a random 256.
        fn random_value(who: &T::AccountId) -> [u8; 16] {
            let nonce = frame_system::Pallet::<T>::account_nonce(&who);
            // let nonce_u32: u32 = nonce as u32;
            // generate a random value based on account and its nonce
            let nonce_u32: u32 = TryInto::try_into(nonce).ok().expect("nonce is u64; qed");
            let a: BlockNumberFor<T> = TryFrom::try_from(nonce_u32)
                .ok()
                .expect("nonce is u32; qed");
            // payload.using_encoded(blake2_128)
            let (random_seed, _) = T::Randomness::random(&who.encode());

            let value = (random_seed, nonce_u32, a);
            let random_res = sp_io::hashing::blake2_128(&value.encode());

            random_res
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

        fn fetch_price() -> Result<u32, http::Error> {
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
            let request = http::Request::get(
                "https://min-api.cryptocompare.com/data/price?fsym=DOT&tsyms=USD",
            );
            let pending = request
                .deadline(deadline)
                .send()
                .map_err(|_| http::Error::IoError)?;
            let response = pending
                .try_wait(deadline)
                .map_err(|_| http::Error::DeadlineReached)??;
            if response.code != 200 {
                log::warn!("Unexpected status code: {}", response.code);
                return Err(http::Error::Unknown);
            }
            let body = response.body().collect::<Vec<u8>>();

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

        fn balance_to_u32(balance: BalanceOf<T>) -> Result<u32, &'static str> {
            // Attempt to convert to u32
            let balance_as_u32: u32 = balance.try_into().map_err(|_| "Conversion failed")?;
            Ok(balance_as_u32)
        }
    }
}
