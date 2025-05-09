use frame_support::pallet_prelude::{Get, GetStorageVersion, StorageVersion};
use sp_weights::Weight;
use crate::{Config, Kitties, Kitty, Pallet};


pub mod v0 {
    use codec::{Decode, Encode};
    use frame_support::pallet_prelude::{MaxEncodedLen, RuntimeDebug, StorageMap, TypeInfo};
    use frame_support::{storage_alias, Blake2_128Concat, Identity};
    use crate::{Config, NextKittyId, Pallet};
    use super::*;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct OldKitty(pub [u8; 16]);

    #[storage_alias]
    pub type Kitties<T: Config, OldKitty> = StorageMap<Pallet<T>, Blake2_128Concat, u32, OldKitty>;

}

pub fn migrate_to_v1<T: Config>() -> Weight {
    let on_chain: StorageVersion = Pallet::<T>::on_chain_storage_version();
    if on_chain == 0 {
        log::info!("current version is 0, will upgrade to v1");
        log::info!(
            "current version is 0, will upgrade to v1, old kitties len: {:?}",
            v0::Kitties::<T, v0::OldKitty>::iter().count()
        );
        Kitties::<T>::translate::<v0::OldKitty, _>(|key: u32, value: v0::OldKitty| {
            log::info!(
              " translate current version is 0, will upgrade to v1, old kitties id: {:?}",
                key
            );
            Some(Kitty{
                dna: value.0,
                price: None
            })
        });
        StorageVersion::new(1).put::<Pallet<T>>();
        let count = Kitties::<T>::iter().count() as u64;
        log::info!(
            "current version is 0, will upgrade to v1, new kitties len: {:?}",
            count
        );
        return T::DbWeight::get().reads_writes(count + 1, count + 1);
    }
    Weight::default()
}