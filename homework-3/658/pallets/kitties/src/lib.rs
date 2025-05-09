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
/// Set the pallet at dev mode for quick PoC. pallet(dev_mode)
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{pallet_prelude::*,traits::{Currency, Randomness, ReservableCurrency}};
    use frame_system::{pallet_prelude::*};
    use serde::{Deserialize, Serialize};
    use sp_std::prelude::*;
    use sp_weights::WeightMeter;

    #[derive(Encode, Decode, TypeInfo, Serialize, Deserialize,Debug,Clone,PartialEq, Eq,MaxEncodedLen)]
    pub struct Kitty(pub [u8; 16]);

    impl Into<[u8; 16]> for crate::pallet::Kitty {
        fn into(self) -> [u8; 16] {
            self.0
        }
    }
    impl From<[u8; 16]> for Kitty {
        fn from(item: [u8; 16]) -> Self {
            Kitty(item)
        }
    }
    pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn kitty_id)]
    pub type KittyId<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, u32, Kitty>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_owner)]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

    // 扩展存储，能得到一个账号拥有的所有kitties
    #[pallet::storage]
    #[pallet::getter(fn owner_kitties)]
    pub type OwnerKitties<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<Kitty, T::MaxKittiesOwned>,
        ValueQuery,
    >;

    // bid price for each kitty,
    #[pallet::storage]
    #[pallet::getter(fn kitties_bid)]
    pub type KittiesBid<T: Config> = StorageMap<_, Blake2_128Concat, u32, Option<(T::AccountId, BalanceOf<T>)>,ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties_on_sale)]
    pub type KittiesOnSale<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BlockNumberFor<T>,
        BoundedVec<u32, T::MaxKittiesBidPerBlock>,
        ValueQuery,
    >;
}
