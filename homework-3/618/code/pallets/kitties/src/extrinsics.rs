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
            // 获取随机值
            let value = Self::random_value(&who);
            // // 从存储中获取
            // let kitty_id=NextKittyId::<T>::get();
            // // 链上存储，用对象存储
            // Kitties::<T>::insert(kitty_id,Kitty(value));
            // KittyOwner::<T>::insert(kitty_id,who.clone());
            // // 下一个加1，可能溢出
            // let next_kitty_id=kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            // // 更新
            // NextKittyId::<T>::put(next_kitty_id);

            
            // Self::deposit_event(Event::KittyCreated { 
            //     creator:who,
            //     index:kitty_id,
            //     data:value,
            //  });
           Self::create_with_stake(&who,value)?;
            Ok(())
        }
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::breed())]
        pub fn breed(origin: OriginFor<T>, kitty_1: u32, kitty_2: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // 判断kitty_1和kitty_2是否有效
            ensure!(Kitties::<T>::contains_key(&kitty_1),Error::<T>::InvalidKittyId);
            ensure!(Kitties::<T>::contains_key(&kitty_2),Error::<T>::InvalidKittyId);
            // 判断kitty_1和kitty_2是否是同一个
            ensure!(kitty_1 !=kitty_2,Error::<T>::SameKittyId);
            // 判断是否是拥有者
            ensure!(Some(who.clone()) == KittyOwner::<T>::get(kitty_1), Error::<T>::NotOwner);
            ensure!(Some(who.clone()) == KittyOwner::<T>::get(kitty_2), Error::<T>::NotOwner);
            // 产生新的kitty
            if let (Some(kitty1), Some(kitty2)) = (Kitties::<T>::get(kitty_1), Kitties::<T>::get(kitty_2)) {
                    let value = Self::breed_kitty(&who, kitty1.0, kitty2.0);
                    Self::create_with_stake(&who,value)?;
                }    
            Ok(())
        }
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(origin: OriginFor<T>, kitty_id: u32,new:T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // 判断kitty是否有效
            ensure!(Kitties::<T>::contains_key(&kitty_id),Error::<T>::InvalidKittyId); 
            // 判断是否是kitty的拥有者
            ensure!(Some(who.clone()) == KittyOwner::<T>::get(kitty_id), Error::<T>::NotOwner);
            // 接受方不是自己
            ensure!(who.clone() != new,Error::<T>::TransferToSelf);

            // 转移
            let stake=T::KittyStake::get();
            T::Currency::reserve(&new, stake).map_err(|_| Error::<T>::NotEnoughForStaking)?;
            T::Currency::unreserve(&who, stake);
            KittyOwner::<T>::insert(kitty_id,new.clone());
            Self::deposit_event(Event::KittyTransfered {
                old_owner: who, 
                new_owner: new, 
                kitty_id:kitty_id,
             });
            Ok(())
        }
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::sale())]
        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
            init_amount:BalanceOf<T>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // 检查kitty是否有效
            ensure!(Kitties::<T>::contains_key(&kitty_id),Error::<T>::InvalidKittyId); 
            // 检查是不是kitty的拥有者
            ensure!(Some(who.clone()) == KittyOwner::<T>::get(kitty_id), Error::<T>::NotOwner);
            // 检查是否已经出售过
            ensure!(!KittyOnSale::<T>::contains_key(&kitty_id),Error::<T>::KittyAlreadyOnSale);
           
            // 添加存储项
            KittyOnSale::<T>::insert(kitty_id,(until_block,init_amount));

            Self::deposit_event(Event::KittyOnSaled {
                owner: who.clone(), 
                kitty_id:kitty_id,
             });

            Ok(())
        }
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::bid())]
        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // 检查kitty是否有效
            ensure!(Kitties::<T>::contains_key(&kitty_id),Error::<T>::InvalidKittyId); 
            // 检查bider不是owner
            ensure!(Some(who.clone()) != KittyOwner::<T>::get(kitty_id), Error::<T>::BidderIsOwner);
            // 检查kitty是否on sale
            ensure!(KittyOnSale::<T>::contains_key(&kitty_id), Error::<T>::KittyNotONSale);

            // if let amount=KittiesBid::<T>::get(kitty_id);
            let stake=T::KittyStake::get();
            if KittiesBid::<T>::iter().count()==0 {
                // KittiesBid 没有存储任何值
                if let Some(owner)= KittyOnSale::<T>::get(kitty_id){
                    let amount=owner.1;
                    // 检查是否比最低出价金额高
                    ensure!(amount<=price,Error::<T>::PriceNotHigh);
                    // 存储
                    KittiesBid::<T>::insert(kitty_id,(&who,price));
                    // 抵押
                    T::Currency::reserve(&who, stake).map_err(|_| Error::<T>::NotEnoughForStaking)?;
                }
            }else{
                // 与之前最高的比较
                // 获取之前最高竞拍价
                if let Some(kitty)=KittiesBid::<T>::get(kitty_id){
                     // 检查余额是否足够
                     let amount=kitty.1;
                     let old_bidder=kitty.0;
                    // 检查是否比最低出价金额高
                    ensure!(amount<=price,Error::<T>::PriceNotHigh);
                     // 存储
                     KittiesBid::<T>::insert(kitty_id,(&who,price));
                     // 抵押
                     T::Currency::reserve(&who, stake).map_err(|_| Error::<T>::NotEnoughForStaking)?;
                     //  撤销old_bidder押金
                     T::Currency::unreserve(&old_bidder, stake);
                }
            }

            Self::deposit_event(Event::KittyBided {
                bidder: who.clone(), 
                kitty_id:kitty_id,
             });
            Ok(())
        }
    }
}

