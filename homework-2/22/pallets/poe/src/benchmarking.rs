//! Benchmarking setup for pallet-poe
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Poe;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use sp_std::vec;

#[benchmarks]
mod benches {
	use super::*;

	#[benchmark]
	fn create_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
		let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

		#[extrinsic_call]
		create_claim(RawOrigin::Signed(caller.clone()), claim.clone());

		assert_eq!(
			Proofs::<T>::get(claim),
			Some((caller, frame_system::Pallet::<T>::block_number()))
		);

		Ok(())
	}

	#[benchmark]
	fn revoke_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
		let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

		Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone());

		#[extrinsic_call]
		revoke_claim(RawOrigin::Signed(caller.clone()), claim.clone());

		Ok(())
	}

	#[benchmark]
	fn transfer_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
		let recipient: T::AccountId = account("recipient", 0, 0);
		let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

		Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone());

		#[extrinsic_call]
		transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), recipient);

		Ok(())
	}

	impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test);
}
