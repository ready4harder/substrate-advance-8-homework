use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_core::H256;

// 辅助函数：创建一个测试用的存证
fn create_claim(claim: &[u8]) -> BoundedVec<u8, <Test as crate::Config>::MaxClaimLength> {
    BoundedVec::<u8, <Test as crate::Config>::MaxClaimLength>::try_from(claim.to_vec()).unwrap()
}

#[test]
fn test_create_claim() {
    new_test_ext().execute_with(|| {
        let claim = create_claim(b"test claim");
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 验证存证已被创建
        assert!(Proofs::<Test>::contains_key(&claim));

        // 验证事件被正确触发
        System::assert_last_event(Event::ClaimCreated { who: 1, claim: claim.clone() }.into());

        // 测试重复创建同一个存证会失败
        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    });
}

#[test]
fn test_revoke_claim() {
    new_test_ext().execute_with(|| {
        let claim = create_claim(b"test claim");
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 测试撤销存证
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 验证存证已被撤销
        assert!(!Proofs::<Test>::contains_key(&claim));

        // 验证事件被正确触发
        System::assert_last_event(Event::ClaimRevoked { who: 1, claim: claim.clone() }.into());

        // 测试撤销不存在的存证会失败
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );

        // 测试非所有者撤销存证会失败
        let another_claim = create_claim(b"another test claim");
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), another_claim.clone()));
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2), another_claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    });
}

#[test]
fn test_transfer_claim() {
    new_test_ext().execute_with(|| {
        let claim = create_claim(b"test claim");
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 测试转移存证
        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), 2, claim.clone()));

        // 验证存证所有权已转移
        let (owner, _) = Proofs::<Test>::get(&claim).unwrap();
        assert_eq!(owner, 2);

        // 验证事件被正确触发
        System::assert_last_event(Event::ClaimTransferred { from: 1, to: 2, claim: claim.clone() }.into());

        // 测试转移不存在的存证会失败
        let non_existent_claim = create_claim(b"non existent claim");
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), 2, non_existent_claim.clone()),
            Error::<Test>::ClaimNotExist
        );

        // 测试非所有者转移存证会失败
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(3), 2, claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    });
}
