use crate::{mock::*, Error, Event, Kitties, KittyOwner};
use frame_support::{assert_noop, assert_ok};
use frame_system::Config;
use super::*;
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
        run_to_block(1);
        let caller=<<Test as Config>::RuntimeOrigin>::signed(0);
        assert_ok!(KittiesModule::create(caller));
        assert_eq!(KittyOwner::<Test>::get(0),Some(0));
        assert_eq!(NextKittyId::<Test>::get(),1);
        System::assert_has_event(Event::KittyCreated{
            creator:0,
            kitty_id:0,
            dna: Kitties::<Test>::get(0).unwrap().0.clone(),
        }.into(), );
    });
}

#[test]
fn it_kitty_id_overflow() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=0;
        let caller=<<Test as Config>::RuntimeOrigin>::signed(alice);
        NextKittyId::<Test>::put(u32::MAX);
        assert_noop!(KittiesModule::create(caller),Error::<Test>::NextKittyIdOverflow);
    });
}



#[test]
fn it_works_breed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));

        run_to_block(3);

        assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(0),0,1));
        assert_eq!(KittyOwner::<Test>::get(2),Some(0));
        assert_eq!(NextKittyId::<Test>::get(),3);


        System::assert_has_event(Event::KittyCreated{
            creator:0,
            kitty_id:0,
            dna:Kitties::<Test>::get(2).unwrap().0.clone(),
        }.into(), );
    });
}







#[test]
fn it_works_transfer() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(0),1,0));
        assert_eq!(KittyOwner::<Test>::get(0),Some(1));


        System::assert_has_event(Event::KittyTransfered{
            from: 0,
            to: 1,
            kitty_id:0,
        }.into(), );
    });
}




#[test]
fn it_transfer_not_kitty_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(1),0,2),Error::<Test>::NotOwner);
    });
}


#[test]
fn it_transfer_to_self() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(0),0,0),Error::<Test>::TransferToSelf);
    });
}


#[test]
fn it_works_sale() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20));
        // 检查存储项
        assert_eq!(KittyOnSale::<Test>::get(0),Some((10,20)));

        System::assert_has_event(Event::KittyOnSaled{
            owner: 0,
            kitty_id:0,
        }.into(), );
    });
}

#[test]
fn it_sale_kitty_not_exit() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20),Error::<Test>::InvalidKittyId);
    });
}

#[test]
fn it_sale_not_kitty_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(1),0,10,20),Error::<Test>::NotOwner);
    });
}

#[test]
fn it_sale_kitty_saled() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20));
        assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20),Error::<Test>::KittyAlreadyOnSale);
    });
}


#[test]
fn it_works_bid() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20));
        run_to_block(3);
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(1),0,30));
        assert_eq!(KittiesBid::<Test>::get(0),Some((1,30)));
        System::assert_has_event(Event::KittyBided {
            bidder: 1,
            kitty_id:0,
        }.into(), );
        run_to_block(4);
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(2),0,50));
        assert_eq!(KittiesBid::<Test>::get(0),Some((2,50)));
        System::assert_has_event(Event::KittyBided {
            bidder: 2,
            kitty_id:0,
        }.into(), );
    });
}

#[test]
fn it_bid_when_kitty_not_exit() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(2),0,50),Error::<Test>::InvalidKittyId);
    });
}


#[test]
fn it_bid_when_bidder_is_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20));
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(0),0,50),Error::<Test>::BidderIsOwner);

    });
}


#[test]
fn it_bid_when_kitty_not_on_sale() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(1),0,50),Error::<Test>::KittyNotOnSale);
    });
}

#[test]
fn it_bid_price_not_high1() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20));
        run_to_block(3);
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(1),0,10),Error::<Test>::PriceTooLow);
    });
}

#[test]
fn it_bid_price_not_high2() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20));
        run_to_block(3);
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(1),0,30));
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(2),0,20),Error::<Test>::PriceTooLow);
    });
}

#[test]
fn it_bid_not_enough_for_staking() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20));
        run_to_block(3);
    });
}

#[test]
fn it_trade_work() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(1);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(0),0,10,20));
        run_to_block(3);
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(1),0,30));
        run_to_block(4);
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(2),0,100));
        run_to_block(15);
        // owner
        assert_eq!(KittyOwner::<Test>::get(0),Some(2));
        // sale
        assert!(!KittyOnSale::<Test>::contains_key(0));
        // bid
        assert!(!KittyOnSale::<Test>::contains_key(0));
    });
}