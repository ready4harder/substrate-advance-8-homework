use super::*;
use frame_support::storage_alias;
use frame_support::weights::Weight;
use frame_support::pallet_prelude::*;
use log;
use serde::{Deserialize,Serialize};
use sp_std::prelude::*;

mod v0{
    use super::*;
     #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize, MaxEncodedLen)]
    pub struct OldKitty(pub [u8; 16]);

    #[storage_alias]
    // 使用Identity意味着存储的键不会经过任何转换，直接用作哈希计算
    // 引用以前存储在kitty里的东西
    pub type Kitties<T:Config,OldKitty>=StorageMap<Pallet<T>,Identity,u32,OldKitty>;

}

// migrate from v0->v1
// 数据迁移
pub fn migrate_to_v1<T:Config>()->Weight{
    // 先判断当时代码运行版本，取得当前版本号,通过on_chain_storage_version获取
    let on_chain=Pallet::<T>::on_chain_storage_version();
    if on_chain==0{
        log::info!("current version is 0,will upgrade to v1");
        // drain:storagemap的方法，用来移除元素，移除时会返回相应的key和value
        for (key,value)in v0::Kitties::<T,v0::OldKitty>::drain(){
            let new_kitty=Kitty{
                dna:value.0,
                price:0,
            };
            Kitties::<T>::insert(key,new_kitty);
        }
        // 版本号更新，否则重复调用
        StorageVersion::new(1).put::<Pallet<T>>();
    }

    Weight::default()

}