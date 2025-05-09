use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let claim2 = BoundedVec::try_from(vec![0, 2]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(2), claim2.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
		assert_eq!(
			Proofs::<Test>::get(&claim2),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
		assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 10);
		assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 10);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let claim2 = BoundedVec::try_from(vec![0, 2]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(2), claim2.clone());

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(2), claim2.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let claim2 = BoundedVec::try_from(vec![0, 2]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(2), claim2.clone());

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim2.clone()));
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let claim2 = BoundedVec::try_from(vec![0, 2]).unwrap();

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim2.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let claim2 = BoundedVec::try_from(vec![0, 2]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(2), claim2.clone());

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(3), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(4), claim2.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let claim2 = BoundedVec::try_from(vec![0, 2]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(2), claim2.clone());

		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 3));
		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim2.clone(), 4));

		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		let bounded_claim2 =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim2.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((3, frame_system::Pallet::<Test>::block_number()))
		);
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim2),
			Some((4, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let claim2 = BoundedVec::try_from(vec![0, 2]).unwrap();

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim2.clone(), 3),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let claim2 = BoundedVec::try_from(vec![0, 2]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(2), claim2.clone());

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(3), claim.clone(), 4),
			Error::<Test>::NotClaimOwner
		);
		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(4), claim.clone(), 5),
			Error::<Test>::NotClaimOwner
		);
	})
}
