#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use frame_support::{BoundedVec, pallet_prelude::*};
use sp_std::vec;

#[benchmarks]
mod benches {
	use super::*;

	#[benchmark]
	fn create_claim(b: Linear<1, {T::MaxClaimLength::get()}>)->Result<(), BenchmarkError> {
		let who: T::AccountId = whitelisted_caller();
		let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

		#[extrinsic_call]
		create_claim(RawOrigin::Signed(who.clone()), claim.clone());

		assert_eq!(
			Proofs::<T>::get(&claim), 
			Some((who, frame_system::Pallet::<T>::block_number()))
		);
		Ok(())
	}

	#[benchmark]
	fn revoke_claim(b: Linear<1, {T::MaxClaimLength::get()}>)->Result<(), BenchmarkError> {
		let who: T::AccountId = whitelisted_caller();
		let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
		
		Pallet::<T>::create_claim(RawOrigin::Signed(who.clone()).into(), claim.clone())?;

		assert_eq!(
			Proofs::<T>::get(&claim), 
			Some((who.clone(), frame_system::Pallet::<T>::block_number()))
		);

		#[extrinsic_call]
		revoke_claim(RawOrigin::Signed(who.clone()), claim.clone());

		assert_eq!(
			Proofs::<T>::get(&claim), 
			None
		);

		Ok(())
	}

	#[benchmark]
	fn transfer_claim(b: Linear<1, {T::MaxClaimLength::get()}>)->Result<(), BenchmarkError> {
		let who: T::AccountId = whitelisted_caller();
		let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

		Pallet::<T>::create_claim(RawOrigin::Signed(who.clone()).into(), claim.clone())?;
		assert_eq!(
			Proofs::<T>::get(&claim), 
			Some((who.clone(), frame_system::Pallet::<T>::block_number()))
		);

		let destination: T::AccountId = account("destination", 0, 0);
		#[extrinsic_call]
		transfer_claim(RawOrigin::Signed(who.clone()), claim.clone(), destination.clone());

		assert_eq!(
			Proofs::<T>::get(&claim), 
			Some((destination, frame_system::Pallet::<T>::block_number()))
		);
		Ok(())
	}
}