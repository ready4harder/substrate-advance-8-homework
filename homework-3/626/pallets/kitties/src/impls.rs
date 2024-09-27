use frame_support::pallet_macros::pallet_section;
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
            let payload = (
                T::Randomness::random_seed(),
                a,
                <frame_system::Pallet<T>>::extrinsic_index()
            );
            payload.using_encoded(sp_io::hashing::blake2_128)
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

        // mint kitty
        fn do_mint(owner: &T::AccountId, dna: [u8; 16]) -> DispatchResult {
            let kitty_id = NextKittyId::<T>::get();
            let next_kitty_id = kitty_id.checked_add(1)
                .ok_or(Error::<T>::NextKittyIdOverflow)?;
            ensure!(!<Kitties<T>>::contains_key(kitty_id), Error::<T>::DuplicateKitty);
            let kitty = Kitty(dna.clone());

            <Kitties<T>>::insert(kitty_id, kitty);
            <KittyOwner<T>>::insert(kitty_id, owner.clone());
            <NextKittyId<T>>::put(next_kitty_id);
            Self::deposit_event(Event::<T>::KittyCreated {
                creator: owner.clone(),
                kitty_id,
                dna
            });
            Ok(())
        }

        // transfer kitty
        fn do_transfer(from: T::AccountId, to: T::AccountId, kitty_id: u32) -> DispatchResult {
            ensure!(from != to, Error::<T>::TransferToSelf);
            ensure!(KittyOwner::<T>::get(kitty_id).as_ref() == Some(&from), Error::<T>::NotOwner);

            <KittyOwner<T>>::insert(kitty_id, to.clone());

            Self::deposit_event(Event::KittyTransfered { from, to, kitty_id });

            Ok(())
        }

        pub fn do_sale(
            who: T::AccountId,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
            init_amount: BalanceOf<T>
        ) -> DispatchResult {
            ensure!(<Kitties<T>>::contains_key(&kitty_id), Error::<T>::InvalidKittyId);
            ensure!(Some(who.clone()) == <KittyOwner::<T>>::get(kitty_id), Error::<T>::NotOwner);
            ensure!(!<KittyOnSale<T>>::contains_key(&kitty_id), Error::<T>::KittyAlreadyOnSale);

            KittyOnSale::<T>::insert(kitty_id, (until_block, init_amount));

            Self::deposit_event(Event::<T>::KittyOnSaled {
                owner: who.clone(),
                kitty_id: kitty_id
            });
            Ok(())
        }
        // bid for kitty
        pub fn do_bid(bidder: T::AccountId, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
            ensure!(<Kitties<T>>::contains_key(&kitty_id), Error::<T>::InvalidKittyId);
            ensure!(Some(bidder.clone()) != <KittyOwner<T>>::get(kitty_id), Error::<T>::BidderIsOwner);
            ensure!(<KittyOnSale<T>>::contains_key(&kitty_id), Error::<T>::KittyNotOnSale);

            if KittiesBid::<T>::iter().count() == 0 {
                if let Some(owner) = KittyOnSale::<T>::get(kitty_id) {
                    let amount = owner.1;
                    ensure!(price >= amount, Error::<T>::PriceTooLow);
                    KittiesBid::<T>::insert(kitty_id, (&bidder, price));
                }
            } else {
                if let Some(kitty) = KittiesBid::<T>::get(kitty_id) {
                    let amount = kitty.1;

                    ensure!(price >= amount, Error::<T>::PriceTooLow);
                    KittiesBid::<T>::insert(kitty_id, (&bidder, price))
                }
            }
            Self::deposit_event(Event::<T>::KittyBided {
                bidder: bidder.clone(),
                kitty_id: kitty_id
            });
            Ok(())
        }


    }
}
