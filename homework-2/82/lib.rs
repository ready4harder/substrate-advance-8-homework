#![cfg_attr(not(feature = "std"), no_std)]

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
    use frame_support::{ensure, pallet_prelude::*};
    use frame_system::{ensure_signed, pallet_prelude::*};

    use crate::WeightInfo;


    #[pallet::pallet]
    // #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, BlockNumberFor<T>)
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimTransfered(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimLengthTooLarge,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks]
    // 为上面定义的Pallet结构体实现钩子函数：Hooks<BlockNumberFor<T>>
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}   // 没有定义钩子函数

    #[pallet::call]
    impl <T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(claim.len() <= T::MaxClaimLength::get() as usize, Error::<T>::ClaimLengthTooLarge);
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            Proofs::<T>::insert(
                &claim,
                (sender.clone(), frame_system::Pallet::<T>::block_number())
            );

            Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::revoke_claim(claim.len() as u32))]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender, claim));

            Ok(().into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::transfer_claim(claim.len() as u32))]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
            target: T::AccountId
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // 获取存证，如果存证不存在则返回错误
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            // 确保调用者是存证的所有者
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            // 更新存证的所有者
            Proofs::<T>::insert(
                &claim,
                (target.clone(), frame_system::Pallet::<T>::block_number())
            );

            // 触发事件
            Self::deposit_event(Event::ClaimTransfered(sender, target, claim));

            Ok(().into())
        }

    }

}