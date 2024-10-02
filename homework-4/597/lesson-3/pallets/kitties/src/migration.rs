use crate::{Config, Kitties, Kitty, Pallet};
use frame_support::{pallet_prelude::*, storage_alias};
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;

pub mod v0 {
    use super::*;

    #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize, MaxEncodedLen)]
    pub struct OldKitty(pub [u8; 16]);

    #[storage_alias]
    pub type Kitties<T: Config, OldKitty> = StorageMap<Pallet<T>, Blake2_128Concat, u64, OldKitty>;
}

pub fn migration_to_v1<T: Config>() -> Weight {
    log::info!("start migration_to_v1");
    let on_chain_version: StorageVersion = Pallet::<T>::on_chain_storage_version();
    let mut weight: Weight = T::DbWeight::get().reads(1); // Weight for reading the storage version

    if on_chain_version == 0 {
        log::info!("do migration_to_v1");
        for (key, value) in v0::Kitties::<T, v0::OldKitty>::drain() {
            let new_kitty = Kitty {
                dna: value.0,
                price: 0,
            };
            Kitties::<T>::insert(key, new_kitty);
        }
        StorageVersion::new(1).put::<Pallet<T>>();

        let item_count: u64 = v0::Kitties::<T, v0::OldKitty>::iter().count() as u64;
        weight = weight
            .saturating_add(T::DbWeight::get().reads_writes(item_count.into(), item_count.into()));
        weight = weight.saturating_add(T::DbWeight::get().writes(1));
    }

    weight
}
