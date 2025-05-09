// 引入
use super::*;
use crate::{mock::*,Error};

// assert_noop表示当前执行没有链上存储改变，assert_ok表示执行时成功的
// BoundedVec表示存证
use frame_support::{assert_noop,assert_ok,BoundedVec,pallet_prelude::Get};

// 校验create_claim
// 表示是个测试用例
#[test]
fn create_claim_works(){
    // 使用mock中定义的new_test_ext()创建测试环境
    new_test_ext().execute_with(||{
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        // 检查create_claim是否能够成功执行
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()));
        // 断言存储项是否是预期的，即是否相同：从proofs里找（proofs是存储用的） 看与
        assert_eq!(
            // 查找与claim相关的信息，claim映射到storagemap
            Proofs::<Test>::get(&claim),
            // 1为交易发送方，并获取当前区块长度
            Some((1,frame_system::Pallet::<Test>::block_number()))
        );

    })
}

// 测试：如果存证已经存在，则不能重复创建存证
#[test]
fn create_claim_failed_when_claim_already_exists(){
    new_test_ext().execute_with(||{
        let claim=BoundedVec::try_from(vec![0,1]).unwrap();
         assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
         ));
         assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()),
            Error::<Test>::ProofAlreadyExist
         );
    })
}

// 测试是否成功撤销存证
#[test]
fn revoke_claim_works(){
    // 使用mock中定义的new_test_ext()创建测试环境
    new_test_ext().execute_with(||{
        // 创建claim
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        // 检查create_claim是否能够成功执行
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()));
        // 检查revoke_claim是否能够成功执行
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1),claim.clone()));
        // 检查存证是否还存在
        assert_eq!(
            Proofs::<Test>::get(&claim),
            None
        );

    })
}

// 测试撤销存证时,当存证未被创建时，存证不存在的情况
#[test]
fn revoke_claim_when_claim_not_create(){
    // 使用mock中定义的new_test_ext()创建测试环境
    new_test_ext().execute_with(||{
        // 创建claim
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        // 检查存证是否还存在
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1),claim.clone()),
            Error::<Test>::ClaimNotExist
        );

    })
}

// // 测试撤销存证时,当存证创建后撤销，存证不存在的情况
// #[test]
// fn revoke_claim_when_claim_revoked(){
//     // 使用mock中定义的new_test_ext()创建测试环境
//     new_test_ext().execute_with(||{
//        // 创建claim
//        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
//        // 检查create_claim是否能够成功执行
//        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()));
//        // 检查revoke_claim是否能够成功执行
//        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1),claim.clone()));
//        // 检查存证是否还存在
//        assert_noop!(
//            PoeModule::revoke_claim(RuntimeOrigin::signed(1),claim.clone()),
//            Error::<Test>::ClaimNotExist
//        );

//     })
// }

// 测试撤销存证时,不是存证的拥有者无法进行撤销
#[test]
fn revoke_claim_when_not_claim_owner(){
    // 使用mock中定义的new_test_ext()创建测试环境
    new_test_ext().execute_with(||{
        // 创建claim
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        // 检查create_claim是否能够成功执行
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()));
        // 检查存证是否还存在
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2),claim.clone()),
            Error::<Test>::NotClaimOwner
        );

    })
}

// 测试是否成功转移存证
#[test]
fn transfer_claim_works(){
    // 使用mock中定义的new_test_ext()创建测试环境
    new_test_ext().execute_with(||{
        // 创建claim
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        // 检查create_claim是否能够成功执行
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()));
        // 检查transfer_claim是否能够成功执行
        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1),claim.clone(),2));
        // 检查是否转移成功
        assert_eq!(
            // 查找与claim相关的信息，claim映射到storagemap
            Proofs::<Test>::get(&claim),
            Some((2,frame_system::Pallet::<Test>::block_number()))
        );
    })
}

// 测试转移存证时，发送方不存在存证的情况
#[test]
fn transfer_claim_not_exist(){
    // 使用mock中定义的new_test_ext()创建测试环境
    new_test_ext().execute_with(||{
        // 创建claim
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        // 检查是否转移成功
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1),claim.clone(),2),
            Error::<Test>::ClaimNotExist
        );
    })
}
// 测试转移存证时，发送方不是存证的拥有者
#[test]
fn transfer_claim_not_claim_owner(){
    // 使用mock中定义的new_test_ext()创建测试环境
    new_test_ext().execute_with(||{
        // 创建claim
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        // 检查create_claim是否能够成功执行
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()));
        // 检查是否转移成功
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2),claim.clone(),3),
            Error::<Test>::NotClaimOwner
        );
    })
}

