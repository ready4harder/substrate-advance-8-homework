#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet{

    use super::*;

    use frame_support::pallet_prelude::*;

    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config{
        #[pallet::constant]
        type MaxClaimLenth: Get<u32>; 

        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    
        type WeightInfo: WeightInfo;
    }

        #[pallet::pallet]
        pub struct Pallet<T>(_);
        
        #[pallet::storage]
        pub type Proofs<T: Config> = StorageMap<
            _,
            Blake2_128Concat,
            BoundedVec<u8,T::MaxClaimLenth>,
            (T::AccountId,BlockNumberFor<T>),
         >;


        #[pallet::event]
        #[pallet::generate_deposit(pub(super) fn deposit_event)]
        pub enum Event<T: Config> {
            ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLenth>),
            ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLenth>),
            ClaimTransferred(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLenth>), // 添加转移存证事件
        }
        

        #[pallet::error]
        pub enum Error<T>{
            ProofAlreadyExist,
            ClaimNotExist,
            NotClaimOwner,
        }

        #[pallet::call]
impl<T: Config> Pallet<T> {
    //现有的创建存证函数
    #[pallet::call_index(0)]
    #[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))] 
    pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLenth>) -> DispatchResult {
        let sender = ensure_signed(origin)?;

        ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

        Proofs::<T>::insert(
            &claim,
            (sender.clone(), frame_system::Pallet::<T>::block_number()),
        );

        Self::deposit_event(Event::ClaimCreated(sender, claim));

        Ok(().into())
    }

    // 现有的撤销存证函数
    #[pallet::call_index(1)]
    #[pallet::weight(T::WeightInfo::revoke_claim())] 
    pub fn revoke_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLenth>) -> DispatchResult {
        let sender = ensure_signed(origin)?;

        let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

        ensure!(owner == sender, Error::<T>::NotClaimOwner);

        Proofs::<T>::remove(&claim);

        Self::deposit_event(Event::ClaimRevoked(sender, claim));

        Ok(().into())
    }

    // 新添加的转移存证函数
    #[pallet::call_index(2)]
    #[pallet::weight(T::WeightInfo::transfer_claim())] 
    pub fn transfer_claim(
        origin: OriginFor<T>,
        claim: BoundedVec<u8, T::MaxClaimLenth>,
        new_owner: T::AccountId,
    ) -> DispatchResult {
        // 检查调用者是否签名
        let sender = ensure_signed(origin)?;

        // 检查存证是否存在
        let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

        // 确保调用者是存证的所有者
        ensure!(owner == sender, Error::<T>::NotClaimOwner);

        // 将存证的所有权转移给新的所有者
        Proofs::<T>::insert(
            &claim,
            (new_owner.clone(), frame_system::Pallet::<T>::block_number()),
        );

        // 触发事件，表示存证所有权已经被转移
        Self::deposit_event(Event::ClaimTransferred(sender, new_owner, claim));

        Ok(().into())
    }
}



    
}