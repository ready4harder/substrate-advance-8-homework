#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

// 需要在Cargo.toml 新增一個sp_std定義
use sp_std::vec;

use frame_support::{BoundedVec, pallet_prelude::Get};

#[benchmarks]
mod benches {
    use super::*;

    #[benchmark]
    fn create_claim(b: Linear<1, {T::MaxClaimLength::get()}>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize].unwrap());

        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller, frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }

    #[benchmark]
    fn revoke_claim() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();  
        let claim_length = T::MaxClaimLength::get();  
        let claim = BoundedVec::try_from(vec![0; claim_length as usize]).unwrap(); 

        Proofs::<T>::insert(&claim, (caller.clone(), frame_system::Pallet::<T>::block_number()));

        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(Proofs::<T>::get(&claim), None);

        Ok(())
    }

    #[benchmark]
    fn transfer_claim() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();  
        let receiver: T::AccountId = whitelisted_caller();  
        let claim_length = T::MaxClaimLength::get(); 
        let claim = BoundedVec::try_from(vec![0; claim_length as usize]).unwrap(); 

        Proofs::<T>::insert(&claim, (caller.clone(), frame_system::Pallet::<T>::block_number()));

        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), receiver.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((receiver.clone(), frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }

}
