#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_macros::import_section;
pub use pallet::*;

use lite_json::JsonValue;
extern crate alloc;
use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};
use sp_runtime::offchain::{http, Duration};

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
mod migration;

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
    use frame_support::traits::{
        BalanceStatus, Currency, Randomness, ReservableCurrency, StorageVersion,
    };
    use frame_support::{pallet_prelude::*, Blake2_128Concat};
    use frame_system::pallet_prelude::*;
    use serde::{Deserialize, Serialize};
    use sp_runtime::traits::ValidateUnsigned;
    // use sp_runtime::traits::Bounded;
    use sp_std::prelude::*;
    use sp_weights::WeightMeter;

    #[allow(dead_code)]
    pub(crate) const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize, MaxEncodedLen)]
    // pub struct Kitty(pub [u8; 16]);
    pub struct Kitty {
        pub dna: [u8; 16],
        pub price: u32,
    }
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type NextKittyId<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, u64, Kitty>;

    #[pallet::storage]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, u64, T::AccountId>;

    // bid price for each kitty,
    #[pallet::storage]
    pub type KittiesBid<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        BoundedVec<(T::AccountId, BalanceOf<T>), ConstU32<500>>,
    >;
    #[pallet::storage]
    pub type KittyWinner<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, (T::AccountId, BalanceOf<T>)>;

    #[pallet::storage]
    pub type KittySale<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, (T::AccountId, BlockNumberFor<T>, BalanceOf<T>)>;

    #[pallet::storage]
    pub type LatestPrice<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;
        fn validate_unsigned(_source: TransactionSource, call: &Call<T>) -> TransactionValidity {
            match call {
                // Validate the `set_latest_price` unsigned call
                Call::set_latest_price_unsigned { price: _ } => {
                    ValidTransaction::with_tag_prefix("PalletKitties")
                        .priority(1) // Set the priority of the transaction
                        .and_provides([b"price_update"])
                        .longevity(3) // Set the number of blocks the transaction is valid for
                        .propagate(true)
                        .build()
                }
                _ => InvalidTransaction::Call.into(),
            }
        }
    }
}
