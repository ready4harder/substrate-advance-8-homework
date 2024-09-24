use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use frame_system::Config;
use frame_support::traits::Currency;

const KITTY_ID: u32 = 0;
const KITTY_ID2: u32 = 1;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

#[test]
fn it_works_create_kitty (){
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 0;

        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_eq!(crate::NextKittyId::<Test>::get(), KITTY_ID);//         
        assert_ok!(Kitties::create(caller));
        assert_eq!(crate::NextKittyId::<Test>::get(), KITTY_ID+1);// 

        let kitty = crate::Kitties::<Test>::get(KITTY_ID).unwrap();

        System::assert_last_event(
            Event::KittyCreated {
                creator: alice,
                index: 0,
                data: kitty.0,
            }
            .into(),
        );
    });
}

#[test]
fn it_works_kitty_id_overflow() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice =0;

        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);

        crate::NextKittyId::<Test>::put(u32::MAX);

        assert_noop!(Kitties::create(caller),Error::<Test>::KittyIdOverflow);
        
    });
}

#[test]
fn it_works_kitty_breed_with_same_kitty_id() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 0;

        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        
        assert_ok!(Kitties::create(caller.clone()));
        
        assert_noop!(Kitties::breed(caller.clone(),KITTY_ID,KITTY_ID),Error::<Test>::SameKittyId);
    });
}

#[test]
fn it_works_kitty_breed_ok() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 0;

        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        Kitties::create(caller.clone());
        Kitties::create(caller.clone());

        let current_kittty_id = crate::NextKittyId::<Test>::get();
        assert_eq!(current_kittty_id,2);
        assert_ok!(Kitties::breed(caller.clone(),KITTY_ID,KITTY_ID2));

        let kitty = crate::Kitties::<Test>::get(current_kittty_id).unwrap();


        System::assert_last_event(
            Event::KittyBreeded {
                creator: alice,
                index: 2,
                parent_index: (0,1),
                data: kitty.0
            }
            .into(),
        );
    });
}


#[test]
fn it_works_kitty_sale_ok() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 0;

        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        Kitties::create(caller.clone());

        assert_ok!(Kitties::sale(caller.clone(),KITTY_ID,10));

        assert_eq!(crate::KittyOnSale::<Test>::get(0),Some(10));

    });
}

#[test]
fn it_works_kitty_sale_not_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = 0;
        let bob = 1;

        let alice_caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        let bob_caller = <<Test as Config>::RuntimeOrigin>::signed(bob);
        Kitties::create(alice_caller.clone());

        assert_noop!(Kitties::sale(bob_caller.clone(),KITTY_ID,10),Error::<Test>::NotOwner);

    });
}

#[test]
fn it_works_kitty_bid_ok(){
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let sale_account = 1;
        let bid_account = 2;
        let bid_account_2 = 2;
        let price = 100;

        let sale_caller = <<Test as Config>::RuntimeOrigin>::signed(sale_account);

        Kitties::create(sale_caller.clone());

        Kitties::sale(sale_caller.clone(),KITTY_ID,10);

        run_to_block(2);
        let bid_caller = <<Test as Config>::RuntimeOrigin>::signed(bid_account);
        let bid_caller_2 = <<Test as Config>::RuntimeOrigin>::signed(bid_account_2);
        
        assert_ok!(Kitties::bid(bid_caller.clone(),KITTY_ID,100));
        assert_ok!(Kitties::bid(bid_caller_2.clone(),KITTY_ID,200));
        
        // assert_eq!(crate::KittiesBid::<Test>::get(KITTY_ID).unwrap(),vec![(bid_caller_2,200u128)]);
        assert_eq!(crate::KittiesBid::<Test>::get(KITTY_ID).unwrap()[0],(bid_account_2,200u128));
    });
}

#[test]
fn it_works_kitty_bid_not_sale(){
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let sale_account = 1;
        let bid_account = 2;

        let sale_caller = <<Test as Config>::RuntimeOrigin>::signed(sale_account);

        Kitties::create(sale_caller.clone());


        run_to_block(2);
        let bid_caller = <<Test as Config>::RuntimeOrigin>::signed(bid_account);

        assert_noop!(Kitties::bid(bid_caller.clone(),KITTY_ID,100),Error::<Test>::KittyNotOnSale);
        
    });
}

#[test]
fn it_works_kitty_bid_blocknumber_expired(){
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let sale_account = 1;
        let bid_account = 2;

        let sale_caller = <<Test as Config>::RuntimeOrigin>::signed(sale_account);

        Kitties::create(sale_caller.clone());
        Kitties::sale(sale_caller.clone(),KITTY_ID,10);

        run_to_block(11);
        let bid_caller = <<Test as Config>::RuntimeOrigin>::signed(bid_account);

        assert_noop!(Kitties::bid(bid_caller.clone(),KITTY_ID,100),Error::<Test>::KittySaleExpired);
        
    });
}


#[test]
fn it_works_kitty_transfer(){
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let sale_account = 1;
        let buyer_account = 2;

        let sale_caller = <<Test as Config>::RuntimeOrigin>::signed(sale_account);
        // create && sale
        Kitties::create(sale_caller.clone());
        Kitties::sale(sale_caller.clone(),KITTY_ID,10);
        let buyer_caller = <<Test as Config>::RuntimeOrigin>::signed(buyer_account);

        //bid
        Kitties::bid(buyer_caller.clone(),KITTY_ID,100);

        run_to_block(11);

        // set initial balance
        let _ = Balances::make_free_balance_be(&buyer_account, 1000);
        let _ = Balances::make_free_balance_be(&sale_account, 1000);
        
        assert_ok!(Kitties::transfer(sale_caller.clone(),KITTY_ID));

        // after transfer, verify the balance
        assert_eq!(Balances::free_balance(buyer_account), 900);
        assert_eq!(Balances::free_balance(sale_account), 1100);

        assert_eq!(crate::KittyOwner::<Test>::get(KITTY_ID),Some(buyer_account));
        
        System::assert_last_event(
            Event::KittyTransfered {
                owner: sale_account,
                to: buyer_account,
                index: KITTY_ID,
            }
            .into(),
        );
    });
}


