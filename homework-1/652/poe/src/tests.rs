// 导入所需模块
use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// 创建存证成功的测试用例
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );

    })
}

// 创建存证失败的测试用例
#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
        assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
    })
}

// 撤销存证成功的测试用例
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
    })
}

// 撤销存证失败的测试用例
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

// 转移存证成功的测试用例
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));
    })
}

// 转移存证失败的测试用例
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
