use frame_support::pallet_macros::pallet_section;

/// Define the implementation of the pallet, like helper functions.
#[pallet_section]
mod impls {
    use sp_std::mem;
    use sp_io::hashing;

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

            let kittie = Kitty(data);
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
    }
}
