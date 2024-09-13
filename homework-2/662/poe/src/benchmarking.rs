#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_support::{pallet_prelude::Get, BoundedVec};
use frame_system::RawOrigin;
use sp_std::vec;

#[benchmarks]
mod benches {
    use frame_benchmarking::BenchmarkParameter::b;
    use super::*;

    /// 创建存证
    #[benchmark]
    fn create_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller.clone(), frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }

    /// 撤销存证
    #[benchmark]
    fn revoke_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

        // #[extrinsic_call]
        Pallet::<T>::create_claim(frame_system::RawOrigin::Signed(caller.clone()).into(), claim.clone())?;

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller.clone(), frame_system::Pallet::<T>::block_number()))
        );

        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(Proofs::<T>::get(&claim), None);

        Ok(())
    }

    /// 转移存证
    #[benchmark]
    fn transfer_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

        // #[extrinsic_call]
        Pallet::<T>::create_claim(frame_system::RawOrigin::Signed(caller.clone()).into(), claim.clone())?;

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller.clone(), frame_system::Pallet::<T>::block_number()))
        );

        let target: T::AccountId = account("recipient", 0, 0);
        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller.clone()), target.clone(), claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim), 
            Some((target, frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }
}
