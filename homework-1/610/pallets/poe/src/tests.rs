use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 5]).unwrap();
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(2),
            claim.clone()
        ));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
        assert_eq!(<<Test as Config>::MaxClaimLenth as Get<u32>>::get(), 10);
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 6]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(5), claim.clone());

        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(5), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 5]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(5), claim.clone());

        assert_ok!(PoeModule::revoke_claim(
            RuntimeOrigin::signed(5),
            claim.clone()
        ));
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 5]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(5), claim.clone());

        let claim_other = BoundedVec::try_from(vec![1, 2, 7]).unwrap();

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(5), claim_other.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 5]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(4), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(3), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 5]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(4), claim.clone());

        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(4),
            claim.clone(),
            2
        ));

        let bounded_claim =
            BoundedVec::<u8, <Test as Config>::MaxClaimLenth>::try_from(claim.clone()).unwrap();
        assert_eq!(
            Proofs::<Test>::get(&bounded_claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 5]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(4), claim.clone());

        let claim_other = BoundedVec::try_from(vec![1, 2, 5, 7]).unwrap();

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(4), claim_other.clone(), 3),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 5]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(3), claim.clone());

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(4), claim.clone(), 2),
            Error::<Test>::NotClaimOwner
        );
    })
}
