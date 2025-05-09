use frame_support::pallet_macros::pallet_section;

/// Define all hooks used in the pallet.
#[pallet_section]
mod hooks {
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            let weight = migration::migration_to_v1::<T>();
            log::info!("migration_to_v1 weight : {}", weight);
            weight
        }

        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            log::info!("Kitties on_initialize at block {:?}", n);

            for (kitty_id, (owner, until_block, _)) in KittySale::<T>::iter() {
                if until_block == n {
                    // handle_sale_expiration
                    let bids = KittiesBid::<T>::get(kitty_id).unwrap_or_default();
                    if !bids.is_empty() {
                        let bid_win = KittyWinner::<T>::get(kitty_id);
                        if let Some((win_bidder, last_bid_price)) = bid_win {
                            for (bidder, bid_price) in bids {
                                if &bidder != &win_bidder {
                                    //refund_unsuccessful_bidders
                                    T::Currency::unreserve(&bidder, bid_price);
                                } else if &bidder == &win_bidder {
                                    //repatriate_reserved_winner
                                    if T::Currency::repatriate_reserved(
                                        &win_bidder,
                                        &owner,
                                        last_bid_price,
                                        BalanceStatus::Free,
                                    )
                                    .is_ok()
                                    {
                                        // 将 Kitty 转移给最高出价者
                                        KittyOwner::<T>::insert(kitty_id, win_bidder.clone());

                                        //更新USD价格
                                        let p = LatestPrice::<T>::get() as u32;
                                        let usd_price_balance =
                                            BalanceOf::<T>::from(p) * last_bid_price;

                                        match Self::balance_to_u32(usd_price_balance) {
                                            Ok(usd_price) => {
                                                if let Some(k1) = Kitties::<T>::get(kitty_id) {
                                                    Kitties::<T>::insert(
                                                        kitty_id,
                                                        Kitty {
                                                            dna: k1.dna,
                                                            price: usd_price,
                                                        },
                                                    );
                                                };
                                            }
                                            Err(err) => {
                                                log::info!("Error converting balance: {}", err);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 清除 上架状态
                    KittySale::<T>::remove(kitty_id);
                    // 保留 对应的出价记录
                    // KittiesBid::<T>::remove(kitty_id);

                    Self::deposit_event(Event::KittySaleEnded { kitty_id, owner });
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

        fn offchain_worker(block_number: BlockNumberFor<T>) {
            log::info!("Kitties offchain_worker at block {:?}", block_number);
            if let Ok(price) = Self::fetch_price() {
                log::info!(
                    "Create an unsigned transaction to update the latest price {}",
                    price
                );
                let call = Call::set_latest_price_unsigned { price };

                // Submit the unsigned transaction
                if SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
                    .is_err()
                {
                    log::error!("Failed to submit unsigned transaction");
                } else {
                    log::info!("Successfully submitted unsigned transaction.");
                }
            }
        }

        // #[cfg(feature = "try-runtime")]
        // fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
        //     unimplemented!()
        // }

        // #[cfg(feature = "try-runtime")]
        // fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
        //     unimplemented!()
        // }

        // #[cfg(feature = "try-runtime")]
        // fn try_state(_n: BlockNumberFor<T>) -> Result<(), TryRuntimeError> {
        //     unimplemented!()
        // }
    }
}
