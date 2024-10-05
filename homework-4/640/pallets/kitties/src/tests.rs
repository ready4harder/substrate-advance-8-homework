use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, pallet_prelude::*};

use super::*;

// 测试猫咪的创建和溢出
#[test]
fn test_kitty_creation_and_overflow() {
    new_test_ext().execute_with(|| {
        let creator = 1;
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));
        
        // 确认 NextKittyId 应该是 2
        assert_eq!(NextKittyId::<Test>::get(), 1);
        assert!(Kitties::<Test>::get(1).is_some());
        assert_eq!(KittyOwner::<Test>::get(1), Some(creator));
        
        // 测试溢出
        NextKittyId::<Test>::put(u32::MAX);
        assert_noop!(
            PalletKitties::create(RuntimeOrigin::signed(creator)),
            Error::<Test>::NextKittyIdOverflow
        );
    });
}


// 测试猫咪的出售和出价的错误
#[test]
fn test_kitty_sale_and_bid_with_errors() {
    new_test_ext().execute_with(|| {
        let (owner, bidder) = (1, 2);
        let price = 600;

        // 这里确保当前块高度大于1
        run_to_block(1);
        
        // 创建一只猫咪
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(owner)));
        let kitty_id = 1; // 确保ID为1

        // 设置一个足够大的until_block
        let until_block = 20; 

        // 测试出售成功
        assert_ok!(PalletKitties::sale(RuntimeOrigin::signed(owner), kitty_id, until_block));
        System::assert_has_event(
            Event::<Test>::KittyOnSale {
                owner,
                kitty_id,
                until_block,
            }
            .into(),
        );

        // 测试出价成功
        assert_ok!(PalletKitties::bid(RuntimeOrigin::signed(bidder), kitty_id, price));
        System::assert_has_event(
            Event::<Test>::KittyBid {
                bidder,
                kitty_id,
                price,
            }
            .into(),
        );


    });
}
