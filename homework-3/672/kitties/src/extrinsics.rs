use frame_support::pallet_macros::pallet_section;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create())]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let value = Self::random_value(&who);
            let kitty_id = Self::next_kitty_id(value)?;

            log::debug!("kitty_id: {:?}", kitty_id);
            log::debug!("value: {:?}", value);

            Kitties::<T>::insert(kitty_id, Kitty(value));
            KittyOwner::<T>::insert(kitty_id, who.clone());

            Self::deposit_event(Event::KittyCreated {
                creator: who,
                index: kitty_id,
                data: value,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::breed())]
        pub fn breed(
            origin: OriginFor<T>,
            kitty_1: KittyIndex,
            kitty_2: KittyIndex,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::kitties_owner(kitty_1) == Some(who.clone()),
                Error::<T>::NotOwner
            );
            if KittyOnSale::<T>::contains_key(kitty_1) {
                return Err(Error::<T>::AlreadyOnSale.into());
            }
            let kitty1 = Self::kitties(kitty_1).ok_or(Error::<T>::InvalidKittyId)?;
            let kitty2 = Self::kitties(kitty_2).ok_or(Error::<T>::InvalidKittyId)?;
            // only different gender kitties can breed
            ensure!(Self::can_breed(kitty1.0, kitty2.0), Error::<T>::SameGender);

            let new_dna = Self::breed_kitty(&who, kitty1.0, kitty2.0);
            let kitty_id = Self::next_kitty_id(new_dna)?;
            Kitties::<T>::insert(kitty_id, Kitty(new_dna));
            KittyOwner::<T>::insert(kitty_id, who.clone());

            if Some(who.clone()) != Self::kitties_owner(kitty_2) {
                T::Currency::transfer(
                    &who,
                    &Self::kitties_owner(kitty_2).unwrap(),
                    T::BreedFee::get(),
                    ExistenceRequirement::KeepAlive,
                )?;
            }

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            kitty_id: KittyIndex,
            to: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                Self::kitties_owner(kitty_id) == Some(who.clone()),
                Error::<T>::NotOwner
            );
            KittyOwner::<T>::insert(kitty_id, to.clone());

            Self::deposit_event(Event::KittyTransferred {
                from: who,
                to,
                index: kitty_id,
            });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::sale())]
        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: KittyIndex,
            until_block: BlockNumberFor<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::kitties_owner(kitty_id) == Some(who.clone()),
                Error::<T>::NotOwner
            );

            KittyOnSale::<T>::insert(kitty_id, until_block);

            Self::deposit_event(Event::KittyOnSale {
                index: kitty_id,
                until: until_block,
            });
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::bid())]
        pub fn bid(
            origin: OriginFor<T>,
            kitty_id: KittyIndex,
            price: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            T::Currency::reserve(&who, price)?;

            KittiesBid::<T>::mutate(kitty_id, |bids| match bids {
                Some(bids) => bids.push((who.clone(), price)),
                None => *bids = Some(vec![(who.clone(), price)]),
            });

            Self::deposit_event(Event::KittyBid {
                bidder: who,
                index: kitty_id,
                price,
            });

            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight({0})]
        pub fn submit_price(origin: OriginFor<T>, price: u32) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let who = ensure_signed(origin)?;
            // Add the price to the on-chain list.
            Self::add_price(Some(who), price);
            Ok(().into())
        }
    }
}
