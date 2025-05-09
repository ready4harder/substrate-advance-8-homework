use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

//成功创建凭证
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
		assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 10);
	})
}

//错误创建凭证
#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

//撤销凭证成功

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
	})
}

//不存在凭证撤销失败

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

//非拥有者撤销凭证失败
#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

//转移凭证成功

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));

		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	})
}


//转移不存在凭证失败
#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

//非拥有者转移凭证失败

#[test]
fn transfer_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	})
}
