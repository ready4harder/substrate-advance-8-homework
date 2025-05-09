use super::*;

use crate::Config;
use frame_support::pallet_prelude::*;

pub mod v0 { 
    use scale_info::TypeInfo;
    use serde::{Deserialize, Serialize};
    use frame_support::storage_alias;
    use frame_support::pallet_prelude::*;
    use sp_std::prelude::*;
    use crate::Config;
    use crate::Pallet;
    
    #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize, MaxEncodedLen)]
    pub struct Kitty_V0(pub [u8; 16]);

    #[storage_alias]
    pub type Kitties<T: Config, Kitty_V0> = StorageMap<Pallet<T>, Blake2_128Concat, u32, Kitty_V0>;
}

pub fn migrate_to_v1<T: Config>() -> Weight {
    let on_chain: StorageVersion = Pallet::<T>::on_chain_storage_version();
    log::info!("on chain version: {:?}", on_chain);
    if on_chain < 2 {
        for (key, value) in v0::Kitties::<T, v0::Kitty_V0>::drain() {
            log::info!("Kitty id {:?} upgrading from V0 to V1", key);
            let new_kitty = Kitty {
                gene: value.0,
                price: 0,
            };
            Kitties::<T>::insert(key, new_kitty);
        }   
        StorageVersion::new(2).put::<Pallet<T>>();         
        Weight::default()
    } else {
        Weight::zero()
    }
}
