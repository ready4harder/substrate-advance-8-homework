use frame_support::pallet_macros::pallet_section;


/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create())]
        pub fn create(origin: OriginFor<T>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let value = Self::random_value(&who);
            let kitty_id = NextKittyId::<T>::get();
            Kitties::<T>::insert(kitty_id, Kitty(value));

            //  Set the owner of the kitty
            KittyOwner::<T>::insert(kitty_id, who.clone());

            let new_kitty_id = NextKittyId::<T>::get();
            let next_kitty_id = kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            NextKittyId::<T>::put(next_kitty_id);
            
            Self::deposit_event(Event::KittyCreated 
                { creator: who, 
                    index: kitty_id, 
                    data: value,
                });
            

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::breed())]
        pub fn breed(
            origin: OriginFor<T>, 
            kitty_id_1: u32, 
            kitty_id_2: u32
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
            let kitty1 = Kitties::<T>::get(kitty_id_1).ok_or(Error::<T>::KittyNotFound)?;
            let kitty2 = Kitties::<T>::get(kitty_id_2).ok_or(Error::<T>::KittyNotFound)?;
        
            let new_kitty_data = Self::breed_kitty(&who, kitty1.0, kitty2.0);
        
            let new_kitty_id = NextKittyId::<T>::get();
            let next_kitty_id = new_kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
        
            let new_kitty = Kitty(new_kitty_data);
            Kitties::<T>::insert(new_kitty_id, new_kitty);
            KittyOwner::<T>::insert(new_kitty_id, who.clone()); 
            NextKittyId::<T>::put(next_kitty_id);
        
            Self::deposit_event(Event::KittyBred {
                breeder: who,
                new_kitty_id,
                kitty_id_1,
                kitty_id_2,
                new_kitty_data,
            });
        
            Ok(())
        }
        

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(origin: OriginFor<T>, 
            to: T::AccountId, 
            kitty_id: u32)
             -> DispatchResult {
            let from = ensure_signed(origin)?;
        
            let kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::KittyNotFound)?;
            ensure!(KittyOwner::<T>::get(&kitty_id) == Some(from.clone()), Error::<T>::NotOwner);
        
            KittyOwner::<T>::insert(&kitty_id, to.clone());
        
            Self::deposit_event(Event::KittyTransferred {
                from,
                to,
                kitty_id,
            });
        
            Ok(())
        }
        

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::sale())]
        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u32,
            until_block: BlockNumberFor<T>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
        
            // Ensure the kitty exists and the caller is the owner
            ensure!(KittyOwner::<T>::get(&kitty_id) == Some(who.clone()), Error::<T>::NotOwner);
        
            // Ensure the kitty is not already on sale
            ensure!(!KittiesOnSale::<T>::contains_key(&kitty_id), Error::<T>::KittyAlreadyOnSale);
        
            // Insert the sale information
            KittiesOnSale::<T>::insert(kitty_id, (who.clone(), until_block));
        
            // Emit the event
            Self::deposit_event(Event::KittyOnSale {
                owner: who,
                kitty_id,
                until_block,
            });
        
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::bid())]
        pub fn bid(
            origin: OriginFor<T>,
            kitty_id: u32,
            price: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            // Ensure the kitty exists 
            ensure!(Kitties::<T>::contains_key(&kitty_id), Error::<T>::KittyNotFound);
            
            // Ensure the kitty is on sale
            ensure!(KittiesOnSale::<T>::contains_key(&kitty_id), Error::<T>::KittyNotForSale);
            
            // Get the sale information
            let (owner, _) = KittiesOnSale::<T>::get(&kitty_id).ok_or(Error::<T>::KittyNotForSale)?;
            
            
            // Ensure the bidder is not the owner
            ensure!(owner != who, Error::<T>::NotOwner);
            
            // Ensure the bid price is greater than zero
            ensure!(price > BalanceOf::<T>::from(0u32), Error::<T>::InvalidBidPrice);
            
            // Check for the current highest bid
            let current_high_bid = KittiesBid::<T>::get(kitty_id).map(|bids| bids.last().map(|b| b.1)).flatten();
            
            if let Some(high_bid) = current_high_bid {
                ensure!(price > high_bid, Error::<T>::BidTooLow); // Ensure the new bid is greater than the current highest bid
            }
            
            // Reserve the bid amount from the bidder's account
            T::Currency::reserve(&who, price)?;
            
            // Update the bids for the kitty
            KittiesBid::<T>::mutate(kitty_id, |bids| match bids {
                Some(bids) => {
                    bids.push((who.clone(), price));
                }
                None => {
                    *bids = Some(vec![(who.clone(), price)]);
                }
            });
            
            // Emit the event
            Self::deposit_event(Event::KittyBid {
                bidder: who.clone(),
                kitty_id,
                price,
            });
            
            Ok(())
        }
        
    }

    
}