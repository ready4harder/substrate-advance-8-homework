#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        #[pallet::constant]
        type MaxClaimLenth: Get<u32>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLenth>,
        (T::AccountId, BlockNumberFor<T>),
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLenth>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLenth>),
        ClaimTransfered(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLenth>),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
    }
    #[pallet::call(weight(<T as Config>::WeightInfo))]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        // #[pallet::weight(0)]
        #[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLenth>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyExist
            );
            Proofs::<T>::insert(
                &claim,
                (sender.clone(), frame_system::Pallet::<T>::block_number()),
            );

            Self::deposit_event(Event::ClaimCreated(sender, claim));
            Ok(().into())
        }

        #[pallet::call_index(1)]
        // #[pallet::weight(0)]
        #[pallet::weight(T::WeightInfo::revoke_claim(claim.len() as u32))]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLenth>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(sender == owner, Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
        }

        #[pallet::call_index(2)]
        // #[pallet::weight(0)]
        #[pallet::weight(T::WeightInfo::transfer_claim(claim.len() as u32))]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLenth>,
            to: T::AccountId,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            let (owner, block_number) =
                Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(from == owner, Error::<T>::NotClaimOwner);
            Proofs::<T>::insert(
                &claim,
                (to.clone(), frame_system::Pallet::<T>::block_number()),
            );
            Self::deposit_event(Event::ClaimTransfered(from, to, claim));
            Ok(().into())
        }
    }
}
