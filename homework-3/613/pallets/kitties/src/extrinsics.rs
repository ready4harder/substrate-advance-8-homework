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
            
            let current_kittty_id = NextKittyId::<T>::get();
            Kitties::<T>::insert(current_kittty_id,Kitty(value));
            let next_kittty_id = current_kittty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            NextKittyId::<T>::put(next_kittty_id);

            KittyOwner::<T>::insert(current_kittty_id,who.clone());
            
             // Emit an event.
            Self::deposit_event(Event::KittyCreated { creator: who, index: current_kittty_id, data: value });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::breed())]
        pub fn breed(origin: OriginFor<T>, kitty_1: u32, kitty_2: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Implement the `breed` extrinsic.
            ensure!(kitty_1 != kitty_2, Error::<T>::SameKittyId);

            let kitty1 = Kitties::<T>::get(kitty_1).ok_or(Error::<T>::KittyNotExist)?;
            let kitty2 = Kitties::<T>::get(kitty_2).ok_or(Error::<T>::KittyNotExist)?;
            

            let value = Self::breed_kitty(&who,kitty1.0, kitty2.0);
            
            let current_kittty_id = NextKittyId::<T>::get();
            Kitties::<T>::insert(current_kittty_id,Kitty(value));
            let next_kittty_id = current_kittty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            NextKittyId::<T>::put(next_kittty_id);

            KittyOwner::<T>::insert(current_kittty_id,who.clone());

              // Emit an event.
            Self::deposit_event(Event::KittyBreeded { creator: who, index: current_kittty_id,parent_index: (kitty_1,kitty_2), data: value });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(origin: OriginFor<T>, kitty_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // implement the `transfer` extrinsic.
            let owner = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
            ensure!(owner == who, Error::<T>::NotOwner);

            // check the kitty sale period is already finish
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(current_block > KittyOnSale::<T>::get(kitty_id).unwrap(),Error::<T>::TransferSaleNotFinish);
            
            // get the highest bidder
            let bidder = KittiesBid::<T>::get(kitty_id).map(|bids| bids.last().unwrap().0.clone()).unwrap_or(who.clone());
            ensure!(bidder != owner, Error::<T>::NotSelfTransfer);

            let price: BalanceOf<T> = KittiesBid::<T>::get(kitty_id).map(|bids| bids.last().unwrap().1.clone()).unwrap_or(BalanceOf::<T>::default());
            
            //transfer the kitty
            T::Currency::transfer(&bidder,&who,price,ExistenceRequirement::KeepAlive)?;

            // update the storage status
            KittyOwner::<T>::insert(kitty_id,bidder.clone());
            KittiesBid::<T>::remove(kitty_id);
            KittyOnSale::<T>::remove(kitty_id);
            
            // emit an event
            Self::deposit_event(Event::KittyTransfered{ owner: who, to: bidder.clone(), index: kitty_id });

            Ok(())
        }
        
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::sale())]
        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
            ensure!(owner == who, Error::<T>::NotOwner);
            KittyOnSale::<T>::insert(kitty_id,until_block);
            
            // emit an event
            Self::deposit_event(Event::KittyListSaled {index: kitty_id, until_block: until_block });                    

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::bid())]
        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(KittyOnSale::<T>::contains_key(kitty_id),Error::<T>::KittyNotOnSale);
            let current_block = <frame_system::Pallet<T>>::block_number();
            
            // check block number
            ensure!(current_block < KittyOnSale::<T>::get(kitty_id).unwrap(),Error::<T>::KittySaleExpired);
            
            // kitty owner not allow bid
            let owner = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
            ensure!(who != owner, Error::<T>::NotSelfTransfer);

            // check price,only allow bid price > current price
            let current_price = KittiesBid::<T>::get(kitty_id).map(|bids| bids.last().unwrap().1.clone()).unwrap_or(BalanceOf::<T>::default());
            ensure!(price > current_price,Error::<T>::NotEnoughBidPrice);

            
            KittiesBid::<T>::mutate(kitty_id, |bids| match bids{
                Some(bids) =>{
                    bids.insert(0,(who.clone(),price));
                }
                None =>{
                    *bids = Some(vec![(who.clone(),price)]);
                }
            });
            
            // emit an event
            Self::deposit_event(Event::KittyBided { bidder: who, index: kitty_id, price: price });

            Ok(())
        }
    }
}
