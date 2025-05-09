#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

/**!
*  ATS 3.9.0版本相关需求的评审、技术设计文档编写及技术评审，目前在开发过程当中，完成20%。计划12号整体提测，23号发版上线。
*
*  服贸会应用姓名调整及生产环境部暑。
*
*  日常线上问题处理和运维相关工作。
*/
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

/// Pallet module
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type MaxClaimLength: Get<u32>;

        type WeightInfo: WeightInfo;

    }

    /// Pallet storage items.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Storage items.
    #[pallet::storage]
    pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::MaxClaimLength>, (T::AccountId, BlockNumberFor<T>)>;

    ///
    /// Events.
    /// #[pallet::generate_deposit(pub (super) fn deposit_event)]作用是生成Pallet的一个实现，调用框架Pallet deposit_event函数
    /// 代码如下：
    /// pub(super) fn deposit_event(event: Event<T>) {
    ///        let event = <<T as Config>::RuntimeEvent as From<Event<T>>>::from(event);
    ///        let event = <<T as Config>::RuntimeEvent as Into<
    ///            <T as frame_system::Config>::RuntimeEvent,
    ///        >>::into(event);
    ///        <frame_system::Pallet<T>>::deposit_event(event)
    ///    }
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimTransfered(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
    }

    /// Errors.
    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExists,
        ClaimNotExists,
        NotClaimOwner,
    }

    /// Pallet相关的钩子函数
    /// 以on_finalize为例：会在调用substrate框架中frame_support::traits::OnFinalize的on_finalize函数之后回调该Hooks中自定义实现的的on_finalize函数.
    ///
    /// 其它的钩子函数:
    /// frame_support::traits::OnIdle,
    /// frame_support::traits::OnPoll,
    /// frame_support::traits::OnRuntimeUpgrade,
    /// frame_support::traits::OnInitialize
    /// frame_support::traits::OffchainWorker
    /// frame_support::traits::IntegrityTest
    ///
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// pallet可调函数
    /// 宏会生成一个枚举【pub enum Call】，枚举中会生成与定义的可调函数对应的枚举成员。枚举的实现类中会调用Pallet中定义的可调函数，
    /// 调用代码：<Pallet<T>>::create_claim(origin, claim).map(Into::into).map_err(Into::into)
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
        #[pallet::call_index(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExists);
            Proofs::<T>::insert(&claim,
                                (_sender.clone(),
                                 frame_system::Pallet::<T>::block_number()));
            Self::deposit_event(Event::ClaimCreated(_sender, claim));

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::revoke_claim(claim.len() as u32))]
        #[pallet::call_index(1)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExists)?;
            ensure!(owner == _sender, Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);
            Self::deposit_event(Event::ClaimRevoked(_sender, claim));

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::transfer_claim(claim.len() as u32))]
        #[pallet::call_index(2)]
        pub fn transfer_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>, dest: T::AccountId) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExists)?;
            ensure!(owner == _sender, Error::<T>::NotClaimOwner);
            Proofs::<T>::insert(&claim, (dest, frame_system::Pallet::<T>::block_number()));

            Self::deposit_event(Event::ClaimTransfered(_sender, claim));
            Ok(())
        }
    }
}
