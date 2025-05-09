#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::{BoundedVec};
use sp_std::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_claim( b: Linear<1,{T::MaxClaimLength::get()}> )->Result<(),BenchmarkError> {
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller, frame_system::Pallet::<T>::block_number()))
        );
        Ok(())
    }

    #[benchmark]
    fn revoke_claim( b: Linear<1,{T::MaxClaimLength::get()}> )->Result<(),BenchmarkError> {
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        Proofs::<T>::insert(&claim, (caller.clone(), frame_system::Pallet::<T>::block_number()));
        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller.clone()), claim.clone());
        assert_eq!(
            Proofs::<T>::get(&claim),
            None
        );
        Ok(())
    }

    #[benchmark]
    fn transfer_claim( b: Linear<1,{T::MaxClaimLength::get()}> )->Result<(),BenchmarkError> {
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        let target: T::AccountId = account("alice", 0, 0);
        Proofs::<T>::insert(&claim, (caller.clone(), frame_system::Pallet::<T>::block_number()));
        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), target.clone());
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((target, frame_system::Pallet::<T>::block_number()))
        );
        Ok(())
    }

}
