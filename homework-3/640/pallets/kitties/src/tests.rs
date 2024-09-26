use crate::{mock::*, Error, Event};
use frame_support::{
    assert_noop, assert_ok,
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement, ReservableCurrency},
};

use super::*;

#[test]
fn test_default_behavior() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(3); // 调整区块数量
    });
}

#[test]
fn test_kitty_sale() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let (owner, bidder, kitty_id, bid_amount, expiry_block) = (1, 3, 2, 600, 12); // 修改参数
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(owner)));

        assert_ok!(PalletKitties::sale(
            RuntimeOrigin::signed(owner),
            kitty_id,
            expiry_block
        ));
        assert_eq!(
            KittiesOnSale::<Test>::get(&expiry_block),
            BoundedVec::<u32, <Test as Config>::MaxKittiesBidPerBlock>::try_from(vec![kitty_id])
                .unwrap()
        );

        run_to_block(2);
        assert_ok!(PalletKitties::bid(
            RuntimeOrigin::signed(bidder),
            kitty_id,
            bid_amount
        ));
        assert_eq!(KittiesBid::<Test>::get(kitty_id), Some((bidder, bid_amount)));

        run_to_block(expiry_block);
        assert_eq!(KittyOwner::<Test>::get(kitty_id), Some(bidder));
        System::assert_has_event(
            Event::<Test>::KittyTransferred {
                from: owner,
                to: bidder,
                kitty_id,
            }
            .into(),
        );
    });
}

#[test]
fn test_sale_with_insufficient_balance() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let (owner, bidder, kitty_id, bid_amount, expiry_block) = (1, 5, 2, 400, 13);

        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(owner)));

        assert_ok!(PalletKitties::sale(
            RuntimeOrigin::signed(owner),
            kitty_id,
            expiry_block
        ));
        assert_eq!(
            KittiesOnSale::<Test>::get(&expiry_block),
            BoundedVec::<u32, <Test as Config>::MaxKittiesBidPerBlock>::try_from(vec![kitty_id])
                .unwrap()
        );

        run_to_block(2);
        let reserve_amount = <<Test as Config>::StakeAmount as Get<u128>>::get();

        let _ = <Test as Config>::Currency::transfer(
            &owner,
            &bidder,
            <Test as Config>::Currency::minimum_balance() + bid_amount + reserve_amount,
            ExistenceRequirement::KeepAlive,
        );
        assert_ok!(PalletKitties::bid(
            RuntimeOrigin::signed(bidder),
            kitty_id,
            bid_amount
        ));
        assert_eq!(KittiesBid::<Test>::get(kitty_id), Some((bidder, bid_amount)));

        <Test as Config>::Currency::unreserve(&bidder, reserve_amount + 4); // 修改解保留金额

        let _ = <Test as Config>::Currency::transfer(
            &bidder,
            &owner,
            reserve_amount + 4,
            ExistenceRequirement::KeepAlive,
        );

        run_to_block(expiry_block);
        assert_eq!(KittyOwner::<Test>::get(kitty_id), Some(owner));
    });
}

#[test]
fn test_kitty_creation() {
    new_test_ext().execute_with(|| {
        let (creator, kitty_id) = (1, 2); // 修改 kitty_id
        let reserved_balance_before = <Test as Config>::Currency::reserved_balance(&creator);
        let free_balance_before = <Test as Config>::Currency::free_balance(&creator);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));
        assert_eq!(NextKittyId::<Test>::get(), kitty_id);
        assert!(Kitties::<Test>::get(kitty_id).is_some());
        assert_eq!(KittyOwner::<Test>::get(kitty_id), Some(creator));
        let stake_amount = <<Test as Config>::StakeAmount as Get<u128>>::get();
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&creator),
            reserved_balance_before + stake_amount
        );
        assert_eq!(
            <Test as Config>::Currency::free_balance(&creator),
            free_balance_before - stake_amount
        );
        System::assert_has_event(
            Event::<Test>::KittyCreated {
                creator,
                kitty_id,
                data: Kitties::<Test>::get(kitty_id).unwrap().0.clone(),
            }
            .into(),
        );
    });
}

#[test]
fn test_creation_overflow() {
    new_test_ext().execute_with(|| {
        let creator = 2; // 修改 creator
        NextKittyId::<Test>::put(u32::MAX);
        assert_noop!(
            PalletKitties::create(RuntimeOrigin::signed(creator)),
            Error::<Test>::NextKittyIdOverflow
        );
    });
}

#[test]
fn test_creation_insufficient_staking_balance() {
    new_test_ext().execute_with(|| {
        let creator = 5; // 修改 creator
        assert_noop!(
            PalletKitties::create(RuntimeOrigin::signed(creator)),
            Error::<Test>::NotEnoughBalanceForStaking
        );
    });
}

#[test]
fn test_breeding_kitties() {
    new_test_ext().execute_with(|| {
        let (creator, first_kitty_id, second_kitty_id, new_kitty_id) = (1, 3, 4, 5); // 修改 kitty_id
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));
        let reserved_balance_before = <Test as Config>::Currency::reserved_balance(&creator);
        let free_balance_before = <Test as Config>::Currency::free_balance(&creator);
        assert_ok!(PalletKitties::breed(
            RuntimeOrigin::signed(creator),
            first_kitty_id,
            second_kitty_id
        ));
        assert_eq!(NextKittyId::<Test>::get(), new_kitty_id);
        let stake_amount = <<Test as Config>::StakeAmount as Get<u128>>::get();
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&creator),
            reserved_balance_before + stake_amount
        );
        assert_eq!(
            <Test as Config>::Currency::free_balance(&creator),
            free_balance_before - stake_amount
        );
        System::assert_has_event(
            Event::<Test>::KittyCreated {
                creator,
                kitty_id: new_kitty_id,
                data: Kitties::<Test>::get(new_kitty_id).unwrap().0.clone(),
            }
            .into(),
        );
    });
}

#[test]
fn test_breeding_same_parent_id() {
    new_test_ext().execute_with(|| {
        let (creator, parent_kitty_id) = (1, 3); // 修改 kitty_id
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));

        assert_noop!(
            PalletKitties::breed(RuntimeOrigin::signed(creator), parent_kitty_id, parent_kitty_id),
            Error::<Test>::SameParentId
        );
    });
}

#[test]
fn test_breeding_non_owner_parent1() {
    new_test_ext().execute_with(|| {
        let (creator, non_owner, parent_kitty_id, other_kitty_id) = (1, 2, 3, 4); // 修改 kitty_id
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(non_owner)));
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));
        assert_noop!(
            PalletKitties::breed(RuntimeOrigin::signed(creator), parent_kitty_id, other_kitty_id),
            Error::<Test>::NotOwner
        );
    });
}

#[test]
fn test_breeding_non_owner_parent2() {
    new_test_ext().execute_with(|| {
        let (creator, non_owner, parent_kitty_id, other_kitty_id) = (1, 2, 3, 4); // 修改 kitty_id
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(non_owner)));
        assert_noop!(
            PalletKitties::breed(RuntimeOrigin::signed(creator), parent_kitty_id, other_kitty_id),
            Error::<Test>::NotOwner
        );
    });
}

#[test]
fn test_breeding_non_existent_kitty1() {
    new_test_ext().execute_with(|| {
        let (creator, first_kitty_id, second_kitty_id) = (1, 3, 4); // 修改 kitty_id
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));
        assert_noop!(
            PalletKitties::breed(RuntimeOrigin::signed(creator), first_kitty_id, second_kitty_id),
            Error::<Test>::KittyNotExist
        );
    });
}

