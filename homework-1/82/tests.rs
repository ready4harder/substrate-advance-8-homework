// 导入所需的模块和类型
use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

// 测试成功创建存证
#[test]
fn test_create_claim_success() {
    new_test_ext().execute_with(|| {
        // 设置区块号
        System::set_block_number(1);

        // 创建一个存证
        let claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::try_from(vec![1_u8, 2_u8, 3_u8, 4_u8]).unwrap();
        let account_id = 1;
        
        // 断言创建存证成功
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(account_id), claim.clone()));

        // 验证存证已被正确存储
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((account_id, frame_system::Pallet::<Test>::block_number()))
        );
    });
}

// 测试创建已存在的存证失败
#[test]
fn test_create_claim_exist_fail() {
    new_test_ext().execute_with(|| {
        // 设置区块号
        System::set_block_number(1);

        // 创建一个存证
        let claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::try_from(vec![1_u8, 2_u8, 3_u8, 4_u8]).unwrap();

        // 首次创建存证成功
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 尝试再次创建相同的存证，应该失败
        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );  
    });
}

// 测试成功撤销存证
#[test]
fn test_revoke_claim_success() {
    new_test_ext().execute_with(|| {
        // 设置区块号
        System::set_block_number(1);

        // 创建一个存证
        let claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::try_from(vec![1_u8, 2_u8, 3_u8, 4_u8]).unwrap();
        // 创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
        // 撤销存证
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
    });
}

// 测试撤销存证失败的情况
#[test]
fn test_revoke_claim_fail() {
    new_test_ext().execute_with(|| {
        // 设置区块号
        System::set_block_number(1);
        // 创建一个存证
        let claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::try_from(vec![1_u8, 2_u8, 3_u8, 4_u8]).unwrap();

        // 创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 尝试用其他账户撤销存证，应该失败
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );

        // 正确撤销存证
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 尝试撤销已撤销的存证，应该失败
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    });
}

// 测试成功转移存证
#[test]
fn test_transfer_claim_success() {
    new_test_ext().execute_with(|| {
        // 设置区块号
        System::set_block_number(1);

        // 创建一个存证
        let claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::try_from(vec![1_u8, 2_u8, 3_u8, 4_u8]).unwrap();
        // 创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
        // 转移存证
        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), 2, claim.clone()));

        // 验证存证已被正确转移
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    });
}

// 测试转移存证失败的情况
#[test]
fn test_transfer_claim_fail() {
    new_test_ext().execute_with(|| {
        // 设置区块号
        System::set_block_number(1);

        // 创建一个存证
        let claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::try_from(vec![1_u8, 2_u8, 3_u8, 4_u8]).unwrap();

        // 创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 尝试转移其他账户的存证，应该失败
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), 3, claim.clone()),
            Error::<Test>::NotClaimOwner
        );

        // 撤销存证
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 尝试转移不存在的存证，应该失败
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), 3, claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    });
}
