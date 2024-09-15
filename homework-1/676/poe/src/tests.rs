// 导入所需模块
use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

//创建成功的测试案例
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        // 使用的存证：
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        //添加成功了
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

// 创建失败的测试案例
#[test]
fn create_claim_failed_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        // new_test_ext 每次都会重置新的测试环境，所以第一次是创建成功的
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        // assert_noop 可以确保如期发生所预期的错误，第一个参数是函数调用，第二个参数是错误类型
        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

// 撤销证据成功的测试案例
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        // new_test_ext 每次都会重置新的测试环境，所以第一次是创建成功的
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        // Ensure the expected error is thrown when no value is present.
        assert_ok!(PoeModule::revoke_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
    });
}

// 撤销证据失败的测试案例:证据不存在；
#[test]
fn revoke_claim_failed_no_claim() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    });
}

// 撤销证据失败的测试案例；证据所有者错误
#[test]
fn revoke_claim_failed_not_right_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    });
}

// 交易证据正确
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));
    });
}

// 交易证据错误：1; 证据不存在

#[test]
fn transfer_claim_failed_no_claim() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
            Error::<Test>::ClaimNotExist
        );
    })
}

// 交易证据错误：2；所有者错误
#[test]
fn transfer_claim_failed_not_right_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
            Error::<Test>::NotClaimOwner
        );
    })
}
