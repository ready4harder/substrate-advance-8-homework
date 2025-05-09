use super::*;
use codec::Decode;
use codec::Encode;
use frame_support::pallet_prelude::StorageVersion;
use frame_support::pallet_prelude::Weight;
use frame_support::storage_alias;
use frame_support::traits::GetStorageVersion;
use frame_support::Identity;
use scale_info::TypeInfo;
use serde::Deserialize;
use serde::Serialize;

mod v0 {
    use super::*;

    #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize)]
    pub struct OldKitty(pub [u8; 16]);

    #[storage_alias]
    pub type Kitties<T: Config, OldKitty> = StorageMap<Pallet<T>, Identity, u32, OldKitty>;
}

// migrate v0 to v1
pub fn migrate_to_v1<T: Config>() -> Weight {
    let on_chain = Pallet::<T>::on_chain_storage_version();

    if on_chain == 0 {
        log::info!("current version is 0, will upgrade to v1");

        for (key, value) in v0::Kitties::<T, v0::OldKitty>::drain() {
            let new_kitty = Kitty {
                dna: value.0,
                price: 0,
            };
            Kitties::<T>::insert(key, new_kitty);
        }

        StorageVersion::new(1).put::<Pallet<T>>();
    }

    Weight::default()
}
