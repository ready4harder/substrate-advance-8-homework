use frame_support::pallet_macros::pallet_section;

/// Define the implementation of the pallet, like helper functions.
#[pallet_section]
mod impls {
    impl<T:  Config> Pallet<T> {
        // get a random 256.
        fn random_value(who: &T::AccountId) -> [u8; 16] {
            let nonce = frame_system::Pallet::<T>::account_nonce(&who);
            // let nonce_u32: u32 = nonce as u32;
            // generate a random value based on account and its nonce
            let nonce_u32: u32 = TryInto::try_into(nonce).ok().expect("nonce is u64; qed");
            let a: BlockNumberFor<T> = TryFrom::try_from(nonce_u32)
                .ok()
                .expect("nonce is u32; qed");
            let payload=(
                T::Randomness::random_seed(),
                a,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            let hash =payload.using_encoded(sp_io::hashing::blake2_128);
            hash
        }

        // breed on kitty based on both paraent kitties
        fn breed_kitty(who: &T::AccountId, kitty_1: [u8; 16], kitty_2: [u8; 16]) -> [u8; 16] {
            let selector = Self::random_value(&who);

            let mut data = [0u8; 16];
            for i in 0..kitty_1.len() {
                // 0 choose kitty2, and 1 choose kitty1
                data[i] = (kitty_1[i] & selector[i]) | (kitty_2[i] & !selector[i]);
            }
            data
        }
        fn create_with_stake(owner:&T::AccountId,v: [u8; 16],price:u32)-> DispatchResult{
            // 从存储中获取
            let kitty_id=NextKittyId::<T>::get();
            // 与balance交互
            // 获取kitty的stake
            let stake=T::KittyStake::get();
            T::Currency::reserve(&owner,stake).map_err(|_|Error::<T>::NotEnoughForStaking)?;

            // 链上存储，用对象存储
            Kitties::<T>::insert(kitty_id,Kitty{
                dna:v,
                price:price,
            });
            KittyOwner::<T>::insert(kitty_id,owner.clone());
            // 下一个加1，可能溢出
            let next_kitty_id=kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            // 更新
            NextKittyId::<T>::put(next_kitty_id);
            
            Self::deposit_event(Event::KittyCreated { 
                creator:owner.clone(),
                index:kitty_id,
                data:v,
             });
             Ok(())
        }
        fn choose_transaction_type(block_number: BlockNumberFor<T>) -> TransactionType {
            const RECENTLY_SENT: () = ();
    
            let val = StorageValueRef::persistent(b"example_ocw::last_send");
            
            let res =
                val.mutate(|last_send: Result<Option<BlockNumberFor<T>>, StorageRetrievalError>| {
                    match last_send {
                        
                        Ok(Some(block)) if block_number < block + T::GracePeriod::get() =>
                            Err(RECENTLY_SENT),
                        _ => Ok(block_number),
                    }
                });
    
            match res {
                
                Ok(block_number) => {
                    let transaction_type = block_number % 4u32.into();
                    assert!(transaction_type == Zero::zero(), "Transaction type must be zero.");
                    TransactionType::Signed
                }
                Err(MutateStorageError::ValueFunctionFailed(RECENTLY_SENT)) => TransactionType::None,
                
                Err(MutateStorageError::ConcurrentModification(_)) => TransactionType::None,
            }
        }
        // 通过fetch_price获取价格，
        fn fetch_price_and_send_signed() -> Result<(), &'static str> {
        // 创建一个signer对象，用于处理签名的相关事务，all_accounts是所有账户
        // 表示singer将管理并能够代表所有可用的本地账户发送签名交易
        let signer = Signer::<T, T::AuthorityId>::all_accounts();
        // 检查是否有可用于签名的本地账户，否则返回错误
        if !signer.can_sign() {
            return Err(
                "No local accounts available. Consider adding one via `author_insertKey` RPC.",
            )
        }
        // 获取价格
        let price = Self::fetch_price().map_err(|_| "Failed to fetch price")?;
        // 为所有可用的账户生成并发送一个包含当前价格的签名交易
        let results = signer.send_signed_transaction(|_account| {
            Call::submit_price { price }
        });
        // 遍历 results 中的每个账户和其对应的结果,检查是否出错
        for (acc, res) in &results {
            match res {
                Ok(()) => log::info!("[{:?}] Submitted price of {} cents", acc.id, price),
                Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
            }
        }

        Ok(())
    }
    // 向外部 API 发起请求以获取比特币到美元的当前价格
    //  http::Error表示与 HTTP 请求和响应相关的错误
        fn fetch_price() -> Result<u32, http::Error> {
        // 获取当前的时间戳，再加上2秒的时间作为请求截止时间，确保请求不会一直等待
        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
        // 创建get请求
        let request =
            http::Request::get("https://min-api.cryptocompare.com/data/price?fsym=DOT&tsyms=USD");
        // request.deadline设置请求截止时间，然后send发送请求
        // send() 方法返回一个 PendingRequest，表示请求仍在进行中
        // map_err 用于将可能发生的错误转换为 http::Error::IoError 类型
        let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;
        
        // 等待请求完成，若请求超时则返回错误
        let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;

        // 请求失败
        if response.code != 200 {
            log::warn!("Unexpected status code: {}", response.code);
            return Err(http::Error::Unknown)
        }
        // 获取响应response中的主体内容，并将其存储在一个字节向量 Vec<u8> 中
        let body = response.body().collect::<Vec<u8>>();
        // 转换为 UTF-8 字符串
        let body_str = alloc::str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            http::Error::Unknown
        })?;
        
        // parse_price解析价格，错误则返回http::Error::Unknown 错误
        let price = match Self::parse_price(body_str) {
            Some(price) => Ok(price),
            None => {
                log::warn!("Unable to extract price from the response: {:?}", body_str);
                Err(http::Error::Unknown)
            },
        }?;
        // 解析成功
        log::warn!("Got price: {} cents", price);
        // 返回以美元为单位的比特币价格
        Ok(price)
    }
    // 解析价格
        fn parse_price(price_str: &str) -> Option<u32> {
        // 解析price_str字符串
        let val = lite_json::parse_json(price_str);
        // val.ok看解析结果是否出错
        let price = match val.ok()? {
            // JsonValue::Object看val是不是一个json类型,是的话把值复制给obj
            JsonValue::Object(obj) => {
                // obj.into_iter().find用来遍历 JSON 对象的键值对
                // find 方法会对迭代器中的每一个元素应用一个闭包，并返回第一个满足条件的元素
                // k.iter().copied()将字符迭代器复制到新的迭代器中,其中eq用来寻找键的值为USD的字符迭代器
                // 找到后返回值给v
                let (_, v) = obj.into_iter().find(|(k, _)| k.iter().copied().eq("USD".chars()))?;
                // 判断v有没有匹配到,匹配到则返回
                match v {
                    JsonValue::Number(number) => number,
                    _ => return None,
                }
            },
            _ => return None,
        };
        // 将货币价格的结构 price 转换为以整数形式表示的价格,如将123.45转化为12345
        let exp = price.fraction_length.saturating_sub(2);
        Some(price.integer as u32 * 100 + (price.fraction / 10_u64.pow(exp)) as u32)
    }

    // 更新价格
        fn add_price(maybe_who: Option<T::AccountId>, price: u32) {
        log::info!("Adding to the average: {}", price);
        // 把新的价格添加到价格列表中去
        // 使用mutate来对Prices进行修改
        // |prices| { ... }：这是一个闭包，prices 是传入的可变引用，代表当前的价格列表
        <Prices<T>>::mutate(|prices| {
            // 将价格添加到列表中，若列表已满，则返回错误
            if prices.try_push(price).is_err() {
                // MaxPrices是指能容下最多的价格数量
                prices[(price % T::MaxPrices::get()) as usize] = price;
            }
        });
        // 记录平均价格
        let average = Self::average_price()
            .expect("The average is not empty, because it was just mutated; qed");
        log::info!("Current average price is: {}", average);
        Self::deposit_event(Event::NewPrice { price, maybe_who });
    }


        fn average_price() -> Option<u32> {
        let prices = Prices::<T>::get();
        if prices.is_empty() {
            None
        } else {
            Some(prices.iter().fold(0_u32, |a, b| a.saturating_add(*b)) / prices.len() as u32)
        }
    }
    }
}