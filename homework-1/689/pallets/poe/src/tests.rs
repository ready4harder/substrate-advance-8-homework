use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_ok() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), Some((1, frame_system::Pallet::<Test>::block_number())));
	})
}

#[test]
fn create_claim_error_duplicate() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
		assert_noop!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim), Error::<Test>::ProofAlreadyExist);
	})
}

#[test]
fn revoke_claim_ok() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim));
	})
}

#[test]
fn revoke_claim_error_no_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
		assert_noop!(PoeModule::revoke_claim(RuntimeOrigin::signed(11), claim), Error::<Test>::NotClaimOwner);
	})
}

#[test]
fn revoke_claim_error_no_existing() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim));
		let claim1 = BoundedVec::try_from(vec![1, 2]).unwrap();
		assert_noop!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim1), Error::<Test>::ClaimNotExist);
	})
}

#[test]
fn transfer_claim_ok() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));
		assert_eq!(Proofs::<Test>::get(&claim), Some((2, frame_system::Pallet::<Test>::block_number())));
	})
}

#[test]
fn transfer_claim_fail_no_existing() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim));
		let claim1 = BoundedVec::try_from(vec![1, 2]).unwrap();
		assert_noop!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim1, 2), Error::<Test>::ClaimNotExist);
	})
}

#[test]
fn transfer_claim_fail_no_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
		assert_noop!(PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3), Error::<Test>::NotClaimOwner);
	})
}