use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
    // 创建一个新的测试环境并执行
    new_test_ext().execute_with(|| {
        // 创建一个 BoundedVec 声明，包含 [0, 1] 的数据，并确认成功
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        // 测试创建声明操作，账户 1 试图创建 claim，预期成功
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        // 验证存储中的声明是否与预期一致，应该存储创建者（账户 1）和当前区块号
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );

        // 验证最大声明长度的配置是否为 10
        assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 10);
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    // 测试重复创建同一声明时是否会失败
    new_test_ext().execute_with(|| {
        // 创建一个声明
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        // 第一次创建应该成功
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        // 再次尝试用相同的账户创建相同的声明，预期会失败，返回 ProofAlreadyExist 错误
        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn revoke_claim_works() {
    // 测试撤销声明的操作
    new_test_ext().execute_with(|| {
        // 创建一个声明
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        // 账户 1 创建该声明
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        // 账户 1 撤销该声明，预期操作成功
        assert_ok!(PoeModule::revoke_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    // 测试撤销不存在的声明时是否会失败
    new_test_ext().execute_with(|| {
        // 创建一个声明，但未存储
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        // 账户 1 尝试撤销未创建的声明，预期返回 ClaimNotExist 错误
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    // 测试错误的所有者尝试撤销声明时是否会失败
    new_test_ext().execute_with(|| {
        // 账户 1 创建了声明
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        // 账户 2 尝试撤销账户 1 的声明，预期返回 NotClaimOwner 错误
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
fn transfer_claim_works() {
    // 测试声明转移的操作
    new_test_ext().execute_with(|| {
        // 账户 1 创建了声明
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        // 账户 1 成功将声明转移给账户 2
        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));

        // 检查存储中的声明是否已更新为账户 2 的所有权
        let bounded_claim =
            BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        assert_eq!(
            Proofs::<Test>::get(&bounded_claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    // 测试转移不存在的声明时是否会失败
    new_test_ext().execute_with(|| {
        // 创建一个声明但未存储
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        // 尝试转移该不存在的声明，预期返回 ClaimNotExist 错误
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
    // 测试错误的所有者尝试转移声明时是否会失败
    new_test_ext().execute_with(|| {
        // 账户 1 创建了声明
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        // 账户 2 尝试将声明转移给账户 3，预期返回 NotClaimOwner 错误
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
            Error::<Test>::NotClaimOwner
        );
    })
}
