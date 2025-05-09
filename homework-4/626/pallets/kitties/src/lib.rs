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

mod config;
mod errors;
mod events;
mod extrinsics;
mod genesis;
mod hooks;
mod impls;
mod migration;
mod validate;

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

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"dot1");
pub mod crypto{
    use super::KEY_TYPE;
    use sp_runtime::{MultiSignature, MultiSigner};
    use sp_runtime::traits::Verify;
    use sp_core::sr25519::Signature as Sr255198Signature;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519}
    };

    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;
    
    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
        
    }

    impl frame_system::offchain::AppCrypto<<Sr255198Signature as Verify>::Signer, Sr255198Signature> for TestAuthId{
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}

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
    use codec::{Decode, Encode};
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency,Randomness};
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use sp_weights::WeightMeter;

    pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Encode, Decode, Clone, PartialEq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    #[codec(mel_bound())]
    pub struct Kitty<T: Config>  {
        pub dna: [u8; 16],
        pub price: Option<BalanceOf<T>>
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type NextKittyId<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, u32, Kitty<T>>;

    #[pallet::storage]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

    #[pallet::storage]
    pub type KittyOnSale<T: Config> = StorageMap<_, Blake2_128Concat, u32,(BlockNumberFor<T>, BalanceOf<T>)>;

    // bid price for each kitty,
    #[pallet::storage]
    pub type KittiesBid<T: Config> = StorageMap<_, Blake2_128Concat, u32, (T::AccountId, BalanceOf<T>)>;

    #[pallet::storage]
    pub(super) type Prices<T: Config> = StorageValue<_, BoundedVec<u32, T::MaxPrices>, ValueQuery>;

    #[pallet::storage]
    pub(super) type NextUnsignedAt<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;
}

/// Payload used by this example crate to hold price
/// data required to submit a transaction.
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

enum TransactionType {
    Signed,
    UnsignedForAny,
    UnsignedForAll,
    Raw,
    None,
}
