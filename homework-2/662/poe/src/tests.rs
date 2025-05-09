
use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

/// 测试成功创建存证
#[test]
fn test_create_claim_success() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let mut claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::new();
        // let raw_vec: Vec<u8> = vec![1_u8, 2_u8, 3_u8];
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        println!("claim: {:?}", claim);
        println!("cliam.len: {:?}", claim.len());
        
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        let (accout_id, block_number) = Proofs::<Test>::get(claim).unwrap();
        println!("accout_id: {}, block_number: {}", accout_id, block_number);
        assert_eq!(accout_id, 1);
    });
}

/// 测试创建存证失败
#[test]
fn test_create_claim_failure() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let mut claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::new();
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 再次创建相同的存证，应该失败
        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    });
}

/// 测试成功撤销存证
#[test]
fn test_revoke_claim_success() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let mut claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::new();
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");

        // 创建存证
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
        // 撤销存证
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
    });
}

/// 测试撤销存证失败
#[test]
fn test_revoke_claim_failure() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let mut claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::new();
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 尝试撤销其他账户的存证，应该失败
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );

        // 撤销不存在的存证，应该失败
        let mut claim2: BoundedVec<u8, ConstU32<4>> = BoundedVec::new();
        claim2.try_push(1_u8).expect("Failed to push");
        claim2.try_push(2_u8).expect("Failed to push");
        claim2.try_push(3_u8).expect("Failed to push");
        claim2.try_push(4_u8).expect("Failed to push");

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim2.clone()),
            Error::<Test>::ClaimNotExist
        );
    });
}

/// 测试成功转移存证
#[test]
fn test_transfer_claim_success() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let mut claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::new();
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), 2, claim.clone()));

        let (accout_id, _block_number) = Proofs::<Test>::get(claim).unwrap();
        assert_eq!(accout_id, 2);
    });
}

/// 测试转移存证失败
#[test]
fn test_transfer_claim_failure() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let mut claim: BoundedVec<u8, ConstU32<4>> = BoundedVec::new();
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");
        claim.try_push(255_u8).expect("Failed to push");

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        // 尝试转移其他账户的存证，应该失败
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), 3, claim.clone()),
            Error::<Test>::NotClaimOwner
        );

        // 转移不存在的存证，应该失败
        let mut claim2: BoundedVec<u8, ConstU32<4>> = BoundedVec::new();
        claim2.try_push(1_u8).expect("Failed to push");
        claim2.try_push(2_u8).expect("Failed to push");
        claim2.try_push(3_u8).expect("Failed to push");
        claim2.try_push(4_u8).expect("Failed to push");

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), 3, claim2.clone()),
            Error::<Test>::ClaimNotExist
        );
    });
}
