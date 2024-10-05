use crate::{mock::*, Error, Event};
use frame_support::{
    assert_noop, assert_ok,
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement, ReservableCurrency},
};
use frame_system::Config;
use crate::NextKittyId;
use crate::KittiesOnSale;

//use sp_std::alloc::System;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

// 針對create kitty的測試
#[test]
fn it_works_create_kitty() {
    new_test_ext().execute_with(|| {
        let (creator, index) = (1, 1);

        // 檢查 kitty 的創建
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));

        // 檢查 event: 判斷有沒有event
        // 所有 event 最終都會放到 System
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


// 針對create kitty 的overflow測試
#[test]
fn it_ketty_id_overflow() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 0;
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);

        // 確保在調用之前已達到最大值
        NextKittyId::<Test>::put(u32::MAX);

        assert_noop!(
            PalletKitties::create(caller), 
            Error::<Test>::KittyIdOverflow
        );
    });
}

// 針對sale kitty 的測試
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
    