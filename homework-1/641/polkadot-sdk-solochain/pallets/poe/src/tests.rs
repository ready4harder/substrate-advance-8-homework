use frame_support::{assert_noop, assert_ok, BoundedVec};

use crate::mock::*;

use super::*;

const SENDER_ALICE: u64 = 1;/// 账户alice
const SENDER_BOB: u64 = 2; /// 账户bob
const SENDER_LILY: u64 = 2; /// 账户lily
const BLOCK_HEIGHT: u64 = 1; /// 区块高度

/// 创建claim的测试数据，方便各个测试用例使用
fn claim_data() -> BoundedVec<u8, <Test as Config>::MaxClaimLength> {
    // 创建一个claim
    let claim = vec![0, 1, 2];
    // 将claim转换成BoundedVec
    claim.try_into().unwrap()
}

#[test]
/// 创建存证用例
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        // 设置区块高度
        System::set_block_number(BLOCK_HEIGHT);

        let claim = claim_data();
        // 将存证添加到链上
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
        // 检查存证是否添加成功，并且区块高度跟设置的一样
        assert_eq!(Proofs::<Test>::get(&claim), Some((SENDER_ALICE, BLOCK_HEIGHT)));
    });
}

#[test]
/// 存证已经存在的用例
fn create_failed_when_claim_exists() {
    new_test_ext().execute_with(|| {
        let claim = claim_data();
        // 先创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
        // 再创建相同的存证，应该返回错误
        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()), Error::<Test>::ProofAlreadyExists
        );
    });
}

#[test]
/// 撤销存证用例
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = claim_data();
        // 先创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
        // 撤销存证
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
    })
}

#[test]
/// 重复撤销存证
/// 并发场景或多页面同时打开操作
fn revoke_claim_failed_when_claim_already_revoked() {
    new_test_ext().execute_with(|| {
        let claim = claim_data();
        // 先创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
        // 再撤销存证
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
        //再次撤销存证，应该返回错误
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()), Error::<Test>::ClaimNotExists
        );
    })
}

#[test]
/// 撤销不存在的存证用例
fn revoke_claim_failed_when_claim_not_exists() {
    new_test_ext().execute_with(|| {
        let claim = claim_data();
        // 撤销不存在的存证，应该返回错误
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()), Error::<Test>::ClaimNotExists
        );
    })
}

#[test]
/// 撤销存证的时候，需要是存证的所有者
fn revoke_claim_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = claim_data();
        // 先创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
        // 再撤销存证，但是不是存证的所有者，应该返回错误
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(SENDER_BOB), claim.clone()), Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
/// 验证存证转移的用例
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        // 设置区块高度
        System::set_block_number(BLOCK_HEIGHT);

        let claim = claim_data();
        // 先创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
        // 再转移存证，新的拥有者为SENDER_BOB
        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone(), SENDER_BOB));

        // 检查存证是否已经从alice转移到bob账户
        assert_ne!(Proofs::<Test>::get(&claim), Some((SENDER_ALICE, BLOCK_HEIGHT)));
        assert_eq!(Proofs::<Test>::get(&claim), Some((SENDER_BOB, BLOCK_HEIGHT)));
    })
}

#[test]
/// 验证存证转移的失败用例，因为不是存证的拥有者
fn transfer_claim_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = claim_data();
        // 先创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
        // 再转移存证，但是不是存证的所有者，应该返回错误
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(SENDER_LILY), claim.clone(), SENDER_BOB), Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
/// 验证存证转移的失败用例，因为存证不存在
fn transfer_claim_failed_when_claim_not_exists() {
    new_test_ext().execute_with(|| {
        let claim = claim_data();
        // 存证不存在，应该返回错误
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone(), SENDER_BOB), Error::<Test>::ClaimNotExists
        );
    })
}

#[test]
/// 验证存证转移的失败用例，因为存证已经被撤销
/// 并发场景为重复撤销
/// 并发场景或多页面同时打开操作
fn transfer_claim_failed_when_claim_already_transfered() {

    new_test_ext().execute_with(|| {
        let claim = claim_data();
        // 先创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone()));
        // 转移存证
        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone(), SENDER_BOB));
        // 再次撤销存证，应该返回错误
        assert_noop!(PoeModule::transfer_claim(RuntimeOrigin::signed(SENDER_ALICE), claim.clone(), SENDER_BOB), Error::<Test>::NotClaimOwner);
    })
}

