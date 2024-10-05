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
            for (kitty_id, (until_block, _)) in KittyOnSale::<T>::iter() {
                if until_block==n{
                    // 获取最后的竞标者
                    if let (Some(new),Some(old_owner))=(KittiesBid::<T>::get(kitty_id),KittyOwner::<T>::get(kitty_id)){
                        let new_owner=new.0;
                        let amount=new.1;
                        let stake_bid=T::KittyStake::get();
                        // 撤销old的押金
                        T::Currency::unreserve(&old_owner,stake_bid);
                        // 转账
                        T::Currency::transfer(
                            &new_owner,
                            &old_owner,
                            amount,
                            ExistenceRequirement::KeepAlive,
                        ).expect("");
                        // // kitty stake 抵押
                        // T::Currency::reserve(&new_owner,stake_kitty).map_err(|_| Error::<T>::NotEnoughForStaking)?;
                        // 修改存储项
                            // 修改owner
                        KittyOwner::<T>::insert(kitty_id,new_owner.clone());
                            // 修改sale
                        KittyOnSale::<T>::remove(kitty_id);
                            // 修改bid
                        KittiesBid::<T>::remove(kitty_id);
                }
                }
            }
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
