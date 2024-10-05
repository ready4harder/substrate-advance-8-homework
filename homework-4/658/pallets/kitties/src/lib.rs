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
extern crate alloc;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use frame_support::traits::Get;
use frame_system::{
    offchain::{
        AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
        SignedPayload, Signer, SigningTypes, SubmitTransaction,
    },
    pallet_prelude::BlockNumberFor,
};
use lite_json::json::JsonValue;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    offchain::{
        http,
        storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
        Duration,
    },
    traits::Zero,
    transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
    RuntimeDebug,
};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"dot!");

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.

mod config;
mod errors;
mod events;
mod extrinsics;
mod genesis;
mod hooks;
mod impls;
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
    #[pallet::storage]
    pub(super) type Prices<T: Config> = StorageValue<_, BoundedVec<u32, T::MaxPrices>, ValueQuery>;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct PricePayload<Public, BlockNumber> {
    block_number: BlockNumber,
    price: u32,
    public: Public,
}
impl<T: SigningTypes> SignedPayload<T> for PricePayload<T::Public, BlockNumberFor<T>> {
    fn public(&self) -> T::Public {
        self.public.clone()
    }
}