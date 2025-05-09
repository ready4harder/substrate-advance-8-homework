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
pub mod offchain;
pub use offchain::*;

/// Import all sections from different files.
#[import_section(extrinsics::dispatches)]
#[import_section(errors::errors)]
#[import_section(events::events)]
#[import_section(config::config)]
#[import_section(hooks::hooks)]
#[import_section(impls::impls)]
#[import_section(genesis::genesis)]
#[import_section(offchain::crypto)]
/// Set the pallet at dev mode for quick PoC.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, Randomness, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use serde::{Deserialize, Serialize};
    use sp_io::hashing::blake2_128;
    use sp_runtime::traits::Bounded;
    use sp_std::prelude::*;
    use sp_weights::WeightMeter;

    pub type Price = u32;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    type KittyIndex = u32;

    #[derive(
        Encode, Decode, Debug, Clone, Default, PartialEq, TypeInfo, Serialize, Deserialize,
    )]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type NextKittyId<T> = StorageValue<_, KittyIndex, ValueQuery>;

    // all the kitties
    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, _, KittyIndex, Kitty>;

    #[pallet::storage]
    #[pallet::getter(fn kitties_owner)]
    pub type KittyOwner<T: Config> = StorageMap<_, _, KittyIndex, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_on_sale)]
    pub type KittyOnSale<T: Config> = StorageMap<_, _, KittyIndex, BlockNumberFor<T>>;

    // bid price for each kitty,
    #[pallet::storage]
    #[pallet::getter(fn kitties_bid)]
    pub type KittiesBid<T: Config> =
        StorageMap<_, _, KittyIndex, Vec<(T::AccountId, BalanceOf<T>)>>;

    #[pallet::storage]
    pub type Prices<T: Config> = StorageValue<_, BoundedVec<Price, T::MaxPrices>, ValueQuery>;
}
