use frame_support::pallet_macros::pallet_section;

/// Define all hooks used in the pallet.
#[pallet_section]
mod hooks {
    use frame_support::traits::ExistenceRequirement;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            Weight::default()
        }

        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            log::info!("Kitties on_initialize at block {:?}", n);

            let mut kitties = Vec::new();
            KittiesSaleInfo::<T>::iter().for_each(|(kitty_id, (_start_price, block_number))| {
                if n > block_number {
                    kitties.push(kitty_id);
                } 
            });

            // remove sale info
            kitties.iter().for_each(|id| KittiesSaleInfo::<T>::remove(id)) ;

            // bid info
            kitties.iter().for_each(|id| {
                let mut v = KittiesBid::<T>::get(id).unwrap();
                
                if !v.is_empty() {
                    let (new_owner, final_price) = v.pop().unwrap();
                    
                    // change kitty owner
                    let prev_owner = KittyOwner::<T>::get(id).unwrap();
                    KittyOwner::<T>::set(id, Some(new_owner.clone()));

                    // reserve/unreserve funds
                    while !v.is_empty() {
                        let (bidder, price) = v.pop().unwrap();
                        T::Currency::unreserve(&bidder, price);
                    }   

                    // transfer fund to previous owner
                    T::Currency::unreserve(&new_owner, final_price);
                    T::Currency::transfer(&new_owner, &prev_owner, final_price, ExistenceRequirement::KeepAlive)
                            .expect(&*format!("can't transfer funds {:#?} of kitty {:?} to previous owner", final_price, id));
                }
            });

            // remove bids info
            kitties.iter().for_each(|id| KittiesBid::<T>::remove(id)) ;

            Weight::default()
        }

        fn on_poll(n: BlockNumberFor<T>, _remaining_weight: &mut WeightMeter) {
            log::info!("Kitties on_poll at block {:?}", n);
        }

        fn on_finalize(n: BlockNumberFor<T>) {
            // remove the kitty on sale if no bid and the block number is greater than the until_block.
            // sale the kitty if according to bid price
            log::info!("Kitties on_finalize at block {:?}", n);
        }

        fn on_idle(n: BlockNumberFor<T>, _remaining_weight: Weight) -> Weight {
            log::info!("Kitties on_idle at block {:?}", n);
            Weight::default()
        }

        fn integrity_test() {
            assert!(NextKittyId::<T>::get() == 0);
        }

        fn offchain_worker(n: BlockNumberFor<T>) {
            log::info!("Kitties offchain_worker at block {:?}", n);
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
            unimplemented!()
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
            unimplemented!()
        }

        #[cfg(feature = "try-runtime")]
        fn try_state(_n: BlockNumberFor<T>) -> Result<(), TryRuntimeError> {
            unimplemented!()
        }
    }
}
