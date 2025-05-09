use frame_support::pallet_macros::pallet_section;

/// Define the implementation of the pallet, like helper functions.
#[pallet_section]
mod impls {
    impl<T: Config> Pallet<T> {
        // get a random 256.
        pub fn random_value(who: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                &who,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }

        // breed on kitty based on both paraent kitties
        pub fn breed_kitty(who: &T::AccountId, kitty_1: [u8; 16], kitty_2: [u8; 16]) -> [u8; 16] {
            let selector = Self::random_value(&who);

            let mut data = [0u8; 16];
            for i in 0..kitty_1.len() {
                // 0 choose kitty2, and 1 choose kitty1
                data[i] = (kitty_1[i] & selector[i]) | (kitty_2[i] & !selector[i]);
            }
            data
        }
    }
}
