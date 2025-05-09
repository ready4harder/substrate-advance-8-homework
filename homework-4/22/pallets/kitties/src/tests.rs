use crate::{mock::*, Error, Event};
use crate::{KittiesOnSale, KittyOwner};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_create() {
    new_test_ext().execute_with(|| {
        assert_eq!(KittyOwner::<Test>::get(0), None);

        assert_ok!(Kitties::create(RuntimeOrigin::signed(1)));

        assert_eq!(KittyOwner::<Test>::get(0), Some(1));
    });
}

#[test]
fn test_breed() {
    new_test_ext().execute_with(|| {
        assert_ok!(Kitties::create(RuntimeOrigin::signed(1)));
        assert_ok!(Kitties::create(RuntimeOrigin::signed(1)));

        assert_eq!(KittyOwner::<Test>::get(0), Some(1));
        assert_eq!(KittyOwner::<Test>::get(1), Some(1));

        assert_ok!(Kitties::breed(RuntimeOrigin::signed(1), 0, 1));

        assert_eq!(KittyOwner::<Test>::get(2), Some(1));
    })
}

#[test]
fn test_transfer() {
    new_test_ext().execute_with(|| {
        assert_ok!(Kitties::create(RuntimeOrigin::signed(1)));
        assert_eq!(KittyOwner::<Test>::get(0), Some(1));

        assert_ok!(Kitties::transfer(RuntimeOrigin::signed(1), 2, 0));
        assert_eq!(KittyOwner::<Test>::get(0), Some(2));
    })
}

#[test]
fn test_sale() {
    new_test_ext().execute_with(|| {
        assert_ok!(Kitties::create(RuntimeOrigin::signed(1)));

        assert_ok!(Kitties::sale(RuntimeOrigin::signed(1), 0, 100));

        assert_eq!(KittiesOnSale::<Test>::get(0), Some(100));
    })
}

#[test]
fn test_bid() {
    new_test_ext().execute_with(|| {
        assert_ok!(Kitties::create(RuntimeOrigin::signed(1)));

        assert_ok!(Kitties::sale(RuntimeOrigin::signed(1), 0, 100));

        run_to_block(2);

        assert_ok!(Kitties::bid(RuntimeOrigin::signed(2), 0, 10));

        run_to_block(100);

        assert_eq!(KittyOwner::<Test>::get(0), Some(2));
    })
}
