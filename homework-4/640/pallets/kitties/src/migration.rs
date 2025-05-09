use crate::{Config, Kitties, Kitty, Pallet};
use frame_support::{pallet_prelude::*, storage_alias};
use sp_std::prelude::*;
// use storage::IterableStorageMap;

mod v0 {
    use super::*;
    // 仅包含 V0 版本的存储格式
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct OldKitty(pub [u8; 16]);
    #[storage_alias]
    pub type Kitties<T: Config, OldKitty> = StorageMap<Pallet<T>, Blake2_128Concat, u32, OldKitty>;
}

// 迁移到 v1 的逻辑
pub fn migrate_to_v1<T: Config>() -> Weight {
    let on_chain = Pallet::<T>::on_chain_storage_version();
    
    if on_chain == 0 {
        log::info!("当前版本是 0，将升级到 v1");
        
        for (key, value) in v0::Kitties::<T, v0::OldKitty>::drain() {
            let new_kitty = Kitty(value.0); 
            Kitties::<T>::insert(key, new_kitty); 
        }
        // 更新存储版本为 1
        StorageVersion::new(1).put::<Pallet<T>>();
    }
    Weight::default()
}
