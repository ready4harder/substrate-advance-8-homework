use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use crate::{Kitties as KittiesArray, NextKittyId, KittyOwner, KittiesBid, KittiesSaleInfo};
use crate::pallet;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

#[test]
fn ok_all() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let bob = 2;
        let charlie = 3;
        
        let alice_balance = <Test as pallet::Config>::Currency::free_balance(&alice);
        let bob_balance = <Test as pallet::Config>::Currency::free_balance(&bob);
        let charlie_balance = <Test as pallet::Config>::Currency::free_balance(&charlie);
        // log::error!("{}", charlie_balance);

        // create two kitties
        assert_ok!(Kitties::create(RuntimeOrigin::signed(alice)));

        assert_eq!(NextKittyId::<Test>::get(), 1);
        assert_eq!(KittiesArray::<Test>::contains_key(0), true);
        assert_eq!(KittyOwner::<Test>::contains_key(0), true);
        assert_eq!(KittyOwner::<Test>::get(0).unwrap(), alice);
        assert_eq!(<Test as pallet::Config>::Currency::free_balance(&alice), alice_balance - 500);
        System::assert_has_event(Event::<Test>::KittyCreated {
                creator: alice,
                index: 0,
                data: Kitties::random_value(&alice),
            }.into());

        assert_ok!(Kitties::create(RuntimeOrigin::signed(alice)));
        assert_eq!(NextKittyId::<Test>::get(), 2);
        assert_eq!(KittiesArray::<Test>::contains_key(1), true);
        assert_eq!(KittyOwner::<Test>::contains_key(1), true);
        assert_eq!(KittyOwner::<Test>::get(1).unwrap(), alice);
        assert_eq!(<Test as pallet::Config>::Currency::free_balance(&alice), alice_balance - 1000);
        System::assert_has_event(Event::<Test>::KittyCreated {
            creator: alice,
            index: 1,
            data: Kitties::random_value(&alice),
        }.into());

        // breed two kitties
        assert_ok!(Kitties::breed(RuntimeOrigin::signed(alice), 0, 1));

        assert_eq!(NextKittyId::<Test>::get(), 3);
        assert_eq!(KittiesArray::<Test>::contains_key(2), true);
        assert_eq!(KittyOwner::<Test>::contains_key(2), true);
        assert_eq!(KittyOwner::<Test>::get(2).unwrap(), alice);
        assert_eq!(<Test as pallet::Config>::Currency::free_balance(&alice), alice_balance - 1500);
        System::assert_has_event(Event::<Test>::KittyCreated {
            creator: alice,
            index: 2,
            data: Kitties::random_value(&alice),
        }.into());

        // transfer a kitty
        assert_ok!(Kitties::transfer(RuntimeOrigin::signed(alice), bob, 0));

        assert_eq!(NextKittyId::<Test>::get(), 3);
        assert_eq!(KittiesArray::<Test>::contains_key(0), true);
        assert_eq!(KittyOwner::<Test>::contains_key(0), true);
        assert_eq!(KittyOwner::<Test>::get(0).unwrap(), bob);
        assert_eq!(<Test as pallet::Config>::Currency::free_balance(&alice), alice_balance - 1000);
        assert_eq!(<Test as pallet::Config>::Currency::free_balance(&bob), bob_balance - 500);
        System::assert_has_event(Event::<Test>::KittyTransferred {
            from: alice,
            to: bob,
            index: 0,
        }.into());

        run_to_block(1);

        // put a kitty on sale
        let until_block = 5;
        assert_ok!(Kitties::sale(RuntimeOrigin::signed(alice), 1, 1000, until_block));

        assert_eq!(KittiesBid::<Test>::contains_key(1), true);
        assert_eq!(KittiesBid::<Test>::get(1).unwrap().is_empty(), true);
        assert_eq!(KittiesSaleInfo::<Test>::contains_key(1), true);
        assert_eq!(KittiesSaleInfo::<Test>::get(1).unwrap(), (1000, until_block));
        System::assert_has_event(Event::<Test>::KittyOnSale {
            index: 1,
            price: 1000,
            until_block,
        }.into());

        // bid for kitty
        assert_ok!(Kitties::bid(RuntimeOrigin::signed(bob), 1, 1000));
        assert_eq!(<Test as pallet::Config>::Currency::free_balance(&bob), bob_balance - 1500);
        assert_ok!(Kitties::bid(RuntimeOrigin::signed(charlie), 1, 1100));
        // log::error!("{:#?}", KittiesBid::<Test>::get(1));
        assert_eq!(KittiesBid::<Test>::contains_key(1), true);
        assert_eq!(KittiesBid::<Test>::get(1).unwrap().len(), 2);
        assert_eq!(<Test as pallet::Config>::Currency::free_balance(&charlie), charlie_balance - 1100);

        // bidding ends
        run_to_block(until_block + 1);

        assert_eq!(KittiesSaleInfo::<Test>::contains_key(1), false);
        assert_eq!(KittiesBid::<Test>::contains_key(1), false);
        assert_eq!(<Test as pallet::Config>::Currency::free_balance(&bob), bob_balance - 500);
        assert_eq!(<Test as pallet::Config>::Currency::free_balance(&charlie), charlie_balance - 1100);
        assert_eq!(KittyOwner::<Test>::get(1).unwrap(), charlie);

    });
}
