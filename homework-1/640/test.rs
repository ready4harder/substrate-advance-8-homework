use super::*;
use crate::{mock::*,Error};

use frame_support::{assert_noop,assert_ok,BoundedVec,pallet_prelude::Get};

#[test]
fn create_claim_works(){
    new_test_ext().execute_with(||{
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1,frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn create_claim_failed_with_claim_already_exist(){
    new_test_ext().execute_with(||{
        let claim= BoundedVec::try_from(vec![0,1]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()));
        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        
        // 首先创建一个存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 然后撤销该存证
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 检查存储中该存证是否已被移除
        assert_eq!(Proofs::<Test>::get(&claim), None);
    });    
}

#[test]
fn revoke_claim_failed_with_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        // 直接尝试撤销一个不存在的存证
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        // 首先由用户1创建一个存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 用户1将该存证转移给用户2
        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));

        // 检查存证的所有权是否已经转移到用户2
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    });
}

#[test]
fn transfer_claim_failed_with_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        // 用户1创建一个存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 用户2尝试将存证转移给用户3，但操作应失败，因为用户2不是该存证的所有者
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
            Error::<Test>::NotClaimOwner
        );
    });
}

