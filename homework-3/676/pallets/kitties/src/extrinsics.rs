use frame_support::pallet_macros::pallet_section;

// use crate::Event::KittyCreated;
// use frame_system::Event;
//use super::events::Event;
//use crate::Event::KittyCreated;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        //创建kitty
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create())]
        pub fn create(origin: OriginFor<T>) -> DispatchResult { 
           
            let who = ensure_signed(origin)?; // 创建者是否已经授权签名

            let _value = Self::random_value(&who); // 假设random_value返回一个随机生成的数值
            let kitty_id: u32 = <NextKittyId<T>>::get().into(); // 获得新的ID号

            // 如果失败的话，返回OVERFLOW；不生成Kitty;  即将溢出，当前尚未溢出；
            let new_kitty_id = kitty_id.checked_add(1).ok_or(Error::<T>::OverFlow)?;

            // 存储新的kitty； Kitty_id+ value
            <KittyInfoList<T>>::insert(kitty_id, KittyInfo(_value));

            //更新Kittyowner
            KittyOwnerList::<T>::insert(kitty_id, who.clone());

            // 更新下一个kitty的ID
            <NextKittyId<T>>::put(new_kitty_id);

            // 发出事件
            Self::deposit_event(Event::KittyCreated {
                creator: who,
                index: kitty_id.into(),
                data: _value,
            });
             
            Ok(())
        }

        // 生产Kitty
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::breed())]
        pub fn breed(origin: OriginFor<T>, kitty_1: u32, kitty_2: u32) -> DispatchResult {  
            let who = ensure_signed(origin)?; // 调用者是否已经授权签名；未签名退出；
            let kitty_id = NextKittyId::<T>::get(); // 保留Kitty_id

            let next_kitty_id = kitty_id.checked_add(1).ok_or(Error::<T>::OverFlow)?; //

            let kitty1 = KittyInfoList::<T>::get(kitty_1).expect("");
            let kitty2 = KittyInfoList::<T>::get(kitty_2).expect("");

            //  生成新的Kitty值
            let value = Self::breed_kitty(&who, kitty1.0, kitty2.0);
            // let value = Self::random_parent_value(kitty_1, kitty_2);

            KittyInfoList::<T>::insert(kitty_id, KittyInfo(value)); //插入Kitty

            NextKittyId::<T>::put(next_kitty_id); // 更新下一个kitty的ID

            //let  who2 = ensure_signed(origin)?; // 调用者是否已经授权签名；未签名退出；
            KittyOwnerList::<T>::insert(kitty_id, who.clone()); //更新kitty所有者MAP

            Self::deposit_event(Event::KittyBreeded {
                creator: who,
                kitty_1: kitty_1,
                kitty_2: kitty_2,
                index: kitty_id.into(),
                data: value,
            });  
            Ok(())
        }

        // 转让Kitty
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, kitty_id: u32) -> DispatchResult {  
            let who = ensure_signed(origin)?;

            let owner = KittyOwnerList::<T>::get(kitty_id);
            // 如果不是所有者则返回：
            match owner {
                Some(owner) => ensure!(who == owner, Error::<T>::NotOwner), // ensure不是requre
                None => ensure!(false, Error::<T>::NotOwner),
            }

            let kitty_info = KittyInfoList::<T>::get(kitty_id);
            // remove
            KittyOwnerList::<T>::remove(kitty_id);

            // insert
            KittyOwnerList::<T>::insert(kitty_id, to.clone()); //更新kitty所有者MAP

            Self::deposit_event(Event::Transfered {
                from: who,
                to: to,
                kitty_id: kitty_id,
            });
              
            Ok(())
        }

        // 出售kitty
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::sale())]
        pub fn sale(
            origin: OriginFor<T>,
            to: T::AccountId,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
        ) -> DispatchResult { 
            let who = ensure_signed(origin)?;

            let owner = KittyOwnerList::<T>::get(kitty_id);
            // 如果不是所有者则返回：
            match owner {
                Some(owner) => ensure!(who == owner, Error::<T>::NotOwner),
                None => ensure!(false, Error::<T>::NotOwner),
            }

            //拍卖成交的区块链高度设置

            let curNum: BlockNumberFor<T> = <frame_system::Pallet<T>>::block_number();

            ensure!(until_block > curNum, Error::<T>::BlockNumberTooSmall);

            //判断是否是结束的拍卖，如果已经结束， 最大值设置0，重新开始拍卖；
            // 如果尚未结束，仅仅修改拍卖结束事件即可：
            //let cur_block_num: BlockNumberFor<T>;
            match MaxBindBlockNum::<T>::get(kitty_id) {
                Some(x) => {
                    // 数据已经存在， 则判断是否已经结束；
                    // 拍卖结束，不能再次竞拍；等待其他流程删除处理
                    ensure!(x > curNum, Error::<T>::SaleIsEnd);
                    //拍卖继续；则更新拍卖结束时间；；其他的不需要处理；
                    //MaxBindBlockNum::<T>::set(kitty_id, until_block); 
                    MaxBindBlockNum::<T>::set(kitty_id, Some(until_block));

                    Self::deposit_event(Event::Saled {
                        from: who,
                        to: to,
                        kitty_id: kitty_id,
                        until_block: until_block,
                    });

                    // 直接返回
                    return Ok(());
                }
                None => {
                    // 数据不存在，直接设置
                    //let some_value: Option<u64> = Some(until_block);
                    //MaxBindValue::<T>::set(kitty_id, Some(until_block));
                    //MaxBindValue::<T>::set(kitty_id, until_block);
                    MaxBindBlockNum::<T>::set(kitty_id, Some(until_block));
                    MaxBindValue::<T>::set(kitty_id, Some(0)); //零价格起拍开始
                }
            }

            // sale

            //KittiesBid::<T>::insert(kitty_id, Option::<(T::AccountId, BalanceOf<T>)>::None);

            KittiesBid::<T>::insert(kitty_id, BoundedVec::new());

            Self::deposit_event(Event::Saled {
                from: who,
                to: to,
                kitty_id: kitty_id,
                until_block: until_block,
            });   
            Ok(())
        }

        //竞拍Kitty
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::bid())]
        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: u64) -> DispatchResult {   
            let who = ensure_signed(origin)?; // 签名检测

            // 取出当前最大的价格
            let MaxValue: u64;

            match MaxBindValue::<T>::get(kitty_id) {
                Some(x) => MaxValue = x,
                None => MaxValue = 0,
            }

            //比当前最大的价格高
            ensure!((price > MaxValue), Error::<T>::InvalidPrice);

            // 读取现有的拍卖条目
            let mut BidEntries = match KittiesBid::<T>::get(kitty_id) {
                Some(x) => x,
                None    => BoundedVec::<(T::AccountId, u64), <T as Config>::MaxBidEntries>::new(),
            };

            // 添加新的拍卖条目
            BidEntries
                .try_push((who, price))
                .map_err(|_| Error::<T>::BidEntriesFull);

            // 存储更新后的拍卖条目
            KittiesBid::<T>::insert(kitty_id, BidEntries);

            // 存储当前最高价格价格
            MaxBindValue::<T>::insert(kitty_id, price);

            // 新的出价事件发出
            Self::deposit_event(Event::OnBid { kitty_id, price });
             
            Ok(())
        }
    }
}
