#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_macros::import_section;
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

/// Import all sections from different files.
#[import_section(extrinsics::dispatches)]
#[import_section(errors::errors)]
#[import_section(events::events)]
#[import_section(config::config)]
#[import_section(hooks::hooks)]
#[import_section(impls::impls)]
#[import_section(genesis::genesis)]
/// Set the pallet at dev mode for quick PoC.

#[frame_support::pallet]

pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency,ExistenceRequirement,Randomness,ReservableCurrency};
    use frame_system::{self as system, pallet_prelude::*};
    use serde::{Deserialize, Serialize};
    use sp_runtime::traits::Bounded;
    use sp_runtime::traits::Block;
    use sp_std::prelude::*;
    use sp_weights::WeightMeter;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize, MaxEncodedLen)]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]
    pub type NextKittyId<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, u32, Kitty>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_owner)]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn kitties_on_sale)]
    pub type KittiesOnSale<T: Config> = StorageMap<_, Blake2_128Concat, u32, (T::AccountId, BlockNumberFor<T>)>;

    // bid price for each kitty,
    #[pallet::storage]
    #[pallet::getter(fn kitties_bid)]
    pub type KittiesBid<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<(T::AccountId, BalanceOf<T>)>>;
}
