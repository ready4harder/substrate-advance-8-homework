use crate::{mock::*, Error, Event};
use frame_support::{
    assert_noop, assert_ok,
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement, ReservableCurrency},
};
use frame_system::Config;
use crate::NextKittyId;
use crate::KittiesOnSale;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

#[test]
fn it_works_create_kitty() {
    new_test_ext().execute_with(|| {
        let (creator, index) = (1, 1);

        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));

        System::assert_has_event(
            Event::<Test>::KittyCreated {
                creator,
                index: 0,
                data: [0_u8; 16],
            }
            .into(),
        );
    });
}


#[test]
fn it_ketty_id_overflow() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 0;
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        NextKittyId::<Test>::put(u32::MAX);
        assert_noop!(
            PalletKitties::create(caller), 
            Error::<Test>::KittyIdOverflow
        );
    });
}

#[test]
fn it_works_for_sale() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let sale_account = 1;
        let bid_account = 2;

        assert_ok!(PalletKitties::create(<<Test as Config>::RuntimeOrigin>::signed(
            sale_account
        )));


        assert_ok!(PalletKitties::sale(
            <<Test as Config>::RuntimeOrigin>::signed(1),
            0,
            10
        ));

        assert_eq!(KittiesOnSale::<Test>::get(0), Some(10));

        run_to_block(2);
        assert_ok!(PalletKitties::bid(
            <<Test as Config>::RuntimeOrigin>::signed(bid_account),
            0,
            100
        ));

        run_to_block(10);

        assert_eq!(KittiesOnSale::<Test>::get(0).unwrap(), 10);
    });
}
    