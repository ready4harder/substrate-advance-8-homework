use frame_support::pallet_macros::pallet_section;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let _value = Self::random_value(&_who);
            let kitty_id = NextKittyId::<T>::get();


            Kitties::<T>::insert(kitty_id, Kitty(_value));

            let next_kitty_id = kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            
            NextKittyId::<T>::put(next_kitty_id);
            KittyOwner::<T>::insert(kitty_id, _who.clone());

            Self::deposit_event(Event::KittyCreated { 
                creator: _who,
                index: kitty_id,
                data: _value,
            });
            
            Ok(())
        }

        pub fn breed(origin: OriginFor<T>, kitty_1: u32, kitty_2: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let kitty1 = Kitties::<T>::get(kitty_1).expect("");
            let kitty2 = Kitties::<T>::get(kitty_2).expect("");

            let value = Self::breed_kitty(&_who, kitty1.0, kitty2.0);

            let kitty_id = NextKittyId::<T>::get();
            Kitties::<T>::insert(kitty_id, Kitty(value));

            let next_kitty_id = kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;

            NextKittyId::<T>::put(next_kitty_id);
            KittyOwner::<T>::insert(kitty_id, _who.clone());

            Self::deposit_event(Event::KittyBreeded {
                creator: _who,
                index: kitty_id,
                data: value,
            });

            Ok(())
        }

        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, kitty_id: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let owner = KittyOwner::<T>::get(kitty_id).expect("");

            ensure!(_who == owner, Error::<T>::NotOwner);

            KittyOwner::<T>::insert(kitty_id, to.clone());

            Self::deposit_event(Event::Transferred {
                from: _who,
                to,
                index: kitty_id,
            });

            Ok(())
        }

        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            KittiesOnSale::<T>::insert(kitty_id, until_block);
            
            Ok(())
        }

        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            T::Currency::reserve(&_who, price)?;

            KittiesBid::<T>::mutate(kitty_id, |bids| match bids {
                Some(bids) => {
                    bids.push((_who.clone(), price));
                }
                None => {
                    *bids = Some(vec![(_who.clone(), price)]);
                }
            });

            Ok(())
        }
    }
}
