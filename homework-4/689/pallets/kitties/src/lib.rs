#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_macros::import_section;
use scale_info::prelude::format;
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

mod config;
mod errors;
mod events;
mod extrinsics;
mod genesis;
mod hooks;
mod impls;
mod migrations;
mod validate;


/// Import all sections from different files.
#[import_section(extrinsics::dispatches)]
#[import_section(errors::errors)]
#[import_section(events::events)]
#[import_section(config::config)]
#[import_section(hooks::hooks)]
#[import_section(impls::impls)]
#[import_section(genesis::genesis)]
#[import_section(validate::validate)]
/// Set the pallet at dev mode for quick PoC.
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Randomness;
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use serde::{Deserialize, Serialize};
    use sp_std::prelude::*;
    use sp_weights::WeightMeter;
    use frame_support::traits::Currency;

    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    
    pub(crate) const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

    #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize, MaxEncodedLen)]
    pub struct Kitty {
        pub gene: [u8; 16],
        pub price: u64,
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type NextKittyId<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, u32, Kitty>;

    #[pallet::storage]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

    // bid price for each kitty,
    #[pallet::storage]
    pub type KittiesBid<T: Config> = StorageMap<_, Blake2_128Concat, u32, BoundedVec<(T::AccountId, BalanceOf<T>), ConstU32<1000>>>;

    #[pallet::storage]
    pub type KittiesSaleInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, (BalanceOf<T>, BlockNumberFor<T>)>;

    #[pallet::storage]
    pub type LatestQuota<T> = StorageValue<_, u64, ValueQuery>;
}
