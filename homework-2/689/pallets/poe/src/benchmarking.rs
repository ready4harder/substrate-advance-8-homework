#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_support::{pallet_prelude::Get, BoundedVec};
use frame_system::RawOrigin;
use sp_std::vec;

#[benchmarks]
mod benches {
    use super::*;

    #[benchmark]
    fn create_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0xff; b as usize]).unwrap();

        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());
        
        assert_eq!(Proofs::<T>::get(&claim), Some((caller, frame_system::Pallet::<T>::block_number())));

        Ok(())
    }

    #[benchmark]
    fn revoke_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0xff; b as usize]).unwrap();

        Pallet::<T>::create_claim(frame_system::RawOrigin::Signed(caller.clone()).into(), claim.clone())?;
        assert_eq!(Proofs::<T>::get(&claim), Some((caller.clone(), frame_system::Pallet::<T>::block_number())));

        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller), claim.clone());

        assert_eq!(Proofs::<T>::get(&claim), None);

        Ok(())
    }   

    #[benchmark]
    fn transfer_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0xff; b as usize]).unwrap();

        Pallet::<T>::create_claim(frame_system::RawOrigin::Signed(caller.clone()).into(), claim.clone())?;
        assert_eq!(Proofs::<T>::get(&claim), Some((caller.clone(), frame_system::Pallet::<T>::block_number())));

        let dest: T::AccountId = account("alice", 0, 0);
        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller.clone()), dest.clone(), claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim), 
            Some((dest, frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }
}
