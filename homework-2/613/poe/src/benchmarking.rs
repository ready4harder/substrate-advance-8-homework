#[cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::v2::*;
use frame_support::{pallet_prelude::Get, BoundedVec};
use frame_system::RawOrigin;
use sp_std::vec;

#[benchmarks]
mod benches {
    use super::*;

    #[benchmark]
    fn create_claim(b: Linear<1, { T::MaxClaimLenth::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0, b as u8]).unwrap();

        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller, frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }

    #[benchmark]
    fn revoke_claim(b: Linear<1, { T::MaxClaimLenth::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0xff, b as u8]).unwrap();

        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone());

        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(Proofs::<T>::get(&claim), None);
        Ok(())
    }

    #[benchmark]
    fn transfer_claim(b: Linear<1, { T::MaxClaimLenth::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0xff, b as u8]).unwrap();
        let target: T::AccountId = account("target", 0, 0);

        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone());

        #[extrinsic_call]
        transfer_claim(
            RawOrigin::Signed(caller.clone()),
            claim.clone(),
            target.clone(),
        );

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((target, frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }
}
