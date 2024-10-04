use frame_support::pallet_macros::pallet_section;
//use crate::migration;
//use crate::migration::migrate_to_v1;
//use frame_support::migration;
use crate::migration;

/// Define all hooks used in the pallet.
#[pallet_section]
mod hooks {
    //use crate::migration;
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            migration::migrate_to_v1::<T>()
            //Weight::default()
        }

        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            log::info!("Kitties on_initialize at block {:?}", n);
            let _ = Self::trade(n);
            /*
            KittiesOnSale::<T>::iter().for_each(
                |(kitty_id, until_block)| {
                    // 判斷until_block是否相等
                    if until_block == n {
                        // 要先找owner, 因為涉及到重後的轉移和交易
                        let owner = KittyOwner::<T>::get(kitty_id).expect("");
                        // 當已找到owner, 就執行下一步, 看有多少人正在拍賣kitty
                        if let Some(bids) = KittiesBid::<T>::take(kitty_id) {
                            // 競拍成功者
                            let mut new_owner = None;
                            // 競拍價格, 由最小的開始
                            let mut final_price = BalanceOf::<T>::min_value();
                            for bid in bids.iter() {
                                T::Currency::unreserve(&bid.0, bid.1);
                                // 如果新價大於最後的價, 僧執行下面動作
                                if bid.1 > final_price {
                                    final_price = bid.1;
                                    new_owner = Some(bid.0.clone());
                                }
                            }

                            if final_price != BalanceOf::<T>::min_value() {
                                T::Currency::transfer(
                                    &new_owner.clone().unwrap(),
                                    &owner,
                                    final_price,
                                    ExistenceRequirement::KeepAlive,
                                )
                                .expect("");

                                // 更新owner
                                KittyOwner::<T>::insert(kitty_id, new_owner.clone().unwrap());
                            }
                        }
                    }
                }
            );
            */
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
            let _ = Self::offchain_worker(n);
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
            log::info!("kitties storage pre_upgrade");
            let kitty_id = NextKittyId::<T>::get();
            Ok(kitty_id.encode())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {
            log::info!("kitties storage post_upgrade");
            let kitty_id_before = u32::decode(&mut &state[..]).map_err(|_| "invalid id state")?;
            assert!(
                kitty_id_before == 0 || Kitties::<T>::contains_key(&kitty_id_before),
                "invalid not include state"
            );
            Ok(())
        }

        #[cfg(feature = "try-runtime")]
        fn try_state(_n: BlockNumberFor<T>) -> Result<(), TryRuntimeError> {
            Ok(())
        }
    }
}
