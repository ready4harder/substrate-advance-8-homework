use frame_support::pallet_macros::pallet_section;

/// Define the implementation of the pallet, like helper functions.
#[pallet_section]
mod impls {

    impl<T: Config> Pallet<T> {
        // get a random 256.
        fn random_value(who: &T::AccountId) -> [u8; 16] {
            log::debug!("============who: {:?}============", who);
            let nonce = frame_system::Pallet::<T>::account_nonce(&who);
            log::debug!("============nonce: {:?}============", nonce);
            // let nonce_u32: u32 = nonce as u32;
            // generate a random value based on account and its nonce
            let nonce_u32: u32 = TryInto::try_into(nonce).ok().expect("nonce is u64; qed");
            let b = frame_system::Pallet::<T>::block_number();
            log::debug!("============block number: {:?}============", b);
            // let p = unsafe { &*(nonce_u32 as *const u32 as *const [u8; 4]) };
            let payload = (T::Randomness::random_seed(), who.clone(), b, nonce_u32);
            payload.using_encoded(blake2_128)
        }

        // breed on kitty based on both parent kitties
        fn breed_kitty(who: &T::AccountId, kitty_1: [u8; 16], kitty_2: [u8; 16]) -> [u8; 16] {
            let selector = Self::random_value(&who);

            let mut data = [0u8; 16];
            for i in 0..kitty_1.len() {
                // 0 choose kitty2, and 1 choose kitty1
                data[i] = (kitty_1[i] & selector[i]) | (kitty_2[i] & !selector[i]);
            }
            data
        }

        fn next_kitty_id(value: [u8; 16]) -> Result<KittyIndex, DispatchError> {
            let kitty_id = NextKittyId::<T>::get();
            Kitties::<T>::insert(kitty_id, Kitty(value));
            let next_kitty_id = kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            NextKittyId::<T>::put(next_kitty_id);
            Ok(kitty_id)
        }

        fn can_breed(kitty_1: [u8; 16], kitty_2: [u8; 16]) -> bool {
            // first bit as gender
            log::debug!("kitty_1: {:?}, kitty_2: {:?}", kitty_1, kitty_2);
            (kitty_1[0] & 1) != (kitty_2[0] & 1)
        }

        fn try_complete_sale(n: BlockNumberFor<T>) -> DispatchResult {
            KittyOnSale::<T>::iter().for_each(|(kitty_id, until_block)| {
                if until_block == n {
                    let owner = Self::kitties_owner(kitty_id).expect("kitty exists");
                    if let Some(bids) = Self::kitties_bid(kitty_id) {
                        let mut bidder = None;
                        let mut highest_bid = BalanceOf::<T>::min_value();
                        for bid in bids {
                            T::Currency::unreserve(&bid.0, bid.1);
                            if bid.1 > highest_bid {
                                highest_bid = bid.1;
                                bidder = Some(bid.0.clone());
                            }
                        }
                        if highest_bid > BalanceOf::<T>::min_value() {
                            T::Currency::transfer(
                                &bidder.clone().unwrap(),
                                &owner,
                                highest_bid,
                                ExistenceRequirement::KeepAlive,
                            )
                            .expect("transfer should succeed; qed");
                            KittyOwner::<T>::insert(kitty_id, bidder.unwrap());
                            KittyOnSale::<T>::remove(kitty_id);
                        }
                    }
                }
            });
            Ok(())
        }
    }
}
