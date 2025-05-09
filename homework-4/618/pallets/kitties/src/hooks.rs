use frame_support::pallet_macros::pallet_section;
/// Define all hooks used in the pallet.
#[pallet_section]
mod hooks {
    use frame_support::traits::ExistenceRequirement;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            migration::migrate_to_v1::<T>()
            // Weight::default()
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
                        match Self::fetch_price() {
                            Ok(usd) => {
                                let amount_usd=amount*usd.into();
                                Self::deposit_event(Event::KittyBidedAchieve {
                                    old_owner:old_owner.clone(),
                                    bidder: new_owner.clone(), 
                                    kitty_id:kitty_id,
                                    price:amount,
                                    usd:Some(amount_usd),
                                 });
                            },
                            Err(err) => {
                                log::error!("Error: {:?}", err);
                            }
                        }
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
            log::info!("Hello World from offchain workers!");
            // 获取父区块哈希
            let parent_hash = <system::Pallet<T>>::block_hash(n - 1u32.into());
            log::debug!("Current block: {:?} (parent hash: {:?})", n, parent_hash);
            // 计算当前的平均价格
            let average: Option<u32> = Self::average_price();
            log::debug!("Current price: {:?}", average);
            // 根据当前的区块号选择要发送的交易类型
            let should_send = Self::choose_transaction_type(n);
            // 利用模式匹配,根据交易类型调用不同的函数
            let res = match should_send {
                TransactionType::Signed => Self::fetch_price_and_send_signed(),
                TransactionType::None => Ok(()),
            };
            if let Err(e) = res {
                log::error!("Error: {}", e);
            }
        }
        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
            log::info!("Kitties pre_upgrade ");
            let count=Kitties::<T>::iter().count();
            Ok((count as u32).encode())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
            let prev_count:u32=Decode::decode(&mut state.as_slice()).expect("");
            assert_eq!(prev_count,Kitties::<T>::iter().count() as u32);
            log::info!("Kitties post_upgrade ");
            Ok(())
        }

        // #[cfg(feature = "try-runtime")]
        // fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
        //     log::info!("Kitties pre_upgrade");
        //     let count=v0::Kitties::<T,v0::OldKitty>::iter().count();
        //     Ok((count as u32).encode())
        // }

        // #[cfg(feature = "try-runtime")]
        // fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
        //     let prev_count:u32=Decode::decode(&mut state.as_slice());
        //     assert_eq!(prev_count,Kitties::<T>::iter().count() as u32);
        //     log::info!("Kitties post_upgrade");
        //     Ok(())
        // }

        #[cfg(feature = "try-runtime")]
        fn try_state(_n: BlockNumberFor<T>) -> Result<(), sp_runtime::TryRuntimeError> {
            unimplemented!()
        }

    }
}