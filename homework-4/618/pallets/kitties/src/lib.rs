#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_macros::import_section;
pub use pallet::*;


extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode};
use frame_support::traits::Get;
use frame_system::{
	self as system,
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

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
// mod crypto;
mod config;
mod errors;
mod events;
mod extrinsics;
mod genesis;
mod hooks;
mod impls;
mod migration;
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"dot!");
pub mod crypto {
    use super::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };
    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;

    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    // implemented for mock runtime in test
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for TestAuthId
    {
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
#[import_section(genesis::genesis)]//初始化
/// Set the pallet at dev mode for quick PoC.
#[frame_support::pallet()]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, Randomness, ReservableCurrency, StorageVersion};
    use frame_system::pallet_prelude::*;
    use serde::{Deserialize, Serialize};
    use sp_std::prelude::*;
    use sp_weights::WeightMeter;

    pub type BalanceOf<T>=<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    #[allow(dead_code)]
    // pub(crate) const STORAGE_VERSION:StorageVersion=StorageVersion::new(0);
    pub(crate) const STORAGE_VERSION:StorageVersion=StorageVersion::new(1);

    // #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize, MaxEncodedLen)]
    // pub struct Kitty(pub [u8; 16);
    
    #[derive(Encode, Decode, Clone, Default, TypeInfo, Serialize, Deserialize, MaxEncodedLen)]
    pub struct Kitty{
        pub dna: [u8; 16],
        pub price: u32,
    }
    enum TransactionType {
        Signed,
        None,
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

    #[pallet::storage]
    pub type KittyOnSale<T: Config> = StorageMap<_, Blake2_128Concat, u32,(BlockNumberFor<T>, BalanceOf<T>)>;

    // bid price for each kitty,
    #[pallet::storage]
    pub type KittiesBid<T: Config> = StorageMap<_, Blake2_128Concat, u32,(T::AccountId, BalanceOf<T>)>;

    #[pallet::storage]
    // 存储一个受限的价格列表，限制了最大元素数量
        pub(super) type Prices<T: Config> = StorageValue<_, BoundedVec<u32, T::MaxPrices>, ValueQuery>;
}