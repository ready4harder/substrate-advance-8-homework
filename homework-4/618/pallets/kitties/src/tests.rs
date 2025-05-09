use crate::{mock::*, Error, Event, Kitties, KittyOwner};
use frame_support::{assert_noop, assert_ok};
use frame_system::Config;
use frame_support::traits::Get;
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
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let caller=<<Test as Config>::RuntimeOrigin>::signed(alice);
        let old_balance=<mock::Test as pallet::Config>::Currency::free_balance(alice);
        let old_stake=<mock::Test as pallet::Config>::Currency::reserved_balance(alice);
        assert_ok!(KittiesModule::create(caller,100));
        // 检查存储项
        assert_eq!(KittyOwner::<Test>::get(0),Some(alice));
        assert_eq!(NextKittyId::<Test>::get(),1);
        // 检查账户变化
        let stake:u128=<mock::Test as pallet::Config>::KittyStake::get();
            // 检查账户金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::free_balance(alice),old_balance-stake);
            // 检查stake金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::reserved_balance(alice),old_stake+stake);
        System::assert_has_event(Event::KittyCreated{
            creator:alice,
            index:0,
            data: Kitties::<Test>::get(0).unwrap().dna.clone(),
        }.into(), );
    });
}

#[test]
fn it_kitty_id_overflow() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let caller=<<Test as Config>::RuntimeOrigin>::signed(alice);
        NextKittyId::<Test>::put(u32::MAX);
        assert_noop!(KittiesModule::create(caller,100),Error::<Test>::KittyIdOverflow);
    });
}

#[test]
fn it_kitty_stake_not_enough() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([5u8; 32]);
        let caller=<<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_noop!(KittiesModule::create(caller,100),Error::<Test>::NotEnoughForStaking);
    });
}


#[test]
fn it_works_breed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));  

        run_to_block(3); 
        let old_balance=<mock::Test as pallet::Config>::Currency::free_balance(alice);
        let old_stake=<mock::Test as pallet::Config>::Currency::reserved_balance(alice);

        assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(alice),0,1,100));
        // 检查存储项
        assert_eq!(KittyOwner::<Test>::get(2),Some(alice));
        assert_eq!(NextKittyId::<Test>::get(),3);
        // 检查账户变化
        let stake:u128=<mock::Test as pallet::Config>::KittyStake::get();
            // 检查账户金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::free_balance(alice),old_balance-stake);
            // 检查stake金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::reserved_balance(alice),old_stake+stake);

        System::assert_has_event(Event::KittyCreated{
            creator:alice,
            index:0,
            data:Kitties::<Test>::get(2).unwrap().dna.clone(),
        }.into(), );
    });
}



#[test]
fn it_breed_when_not_kitties_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(bob),100));  

        run_to_block(3); 
        assert_noop!(KittiesModule::breed(RuntimeOrigin::signed(alice),0,1,100),Error::<Test>::NotOwner);
        
    });
}

#[test]
fn it_breed_when_kitty_not_exist() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(bob),100));  
        run_to_block(3); 
        assert_noop!(KittiesModule::breed(RuntimeOrigin::signed(alice),0,1,100),Error::<Test>::NotOwner);
        
    });
}

#[test]
fn it_breed_when_stake_not_enough() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));  
        run_to_block(3); 
        // 将账号0的所有可用余额（不包括押金）转移到账号1
        assert_ok!(<mock::Test as pallet::Config>::Currency::transfer_all(RuntimeOrigin::signed(alice),bob,true));
        assert_noop!(KittiesModule::breed(RuntimeOrigin::signed(alice),0,1,100),Error::<Test>::NotEnoughForStaking);
    });
}


#[test]
fn it_works_transfer() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        let old_balance=<mock::Test as pallet::Config>::Currency::free_balance(bob);
        let old_stake=<mock::Test as pallet::Config>::Currency::reserved_balance(bob);

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(alice),0,bob));
        // 检查存储项
        assert_eq!(KittyOwner::<Test>::get(0),Some(bob));
        // 检查账户变化
        let stake:u128=<mock::Test as pallet::Config>::KittyStake::get();
            // 检查账户金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::free_balance(bob),old_balance-stake);
            // 检查stake金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::reserved_balance(bob),old_stake+stake);

        System::assert_has_event(Event::KittyTransfered{
            old_owner: alice, 
            new_owner: bob, 
            kitty_id:0,
        }.into(), );
    });
}


#[test]
fn it_transfer_kitty_not_exit() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(alice),0,bob),Error::<Test>::InvalidKittyId);

    });
}


#[test]
fn it_transfer_not_kitty_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        let tom=sp_core::sr25519::Public::from_raw([2u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(bob),0,tom),Error::<Test>::NotOwner);
    });
}


#[test]
fn it_transfer_not_enough_for_sake() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2);  
        // 将账号1的所有可用余额（不包括押金）转移到账号0
        assert_ok!(<mock::Test as pallet::Config>::Currency::transfer_all(RuntimeOrigin::signed(bob),alice,true));
        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(alice),0,bob),Error::<Test>::NotEnoughForStaking);
    });
}

#[test]
fn it_transfer_to_self() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2);  
        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(alice),0,alice),Error::<Test>::TransferToSelf);
    });
}


#[test]
fn it_works_sale() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20));
        // 检查存储项
        assert_eq!(KittyOnSale::<Test>::get(0),Some((10,20)));

        System::assert_has_event(Event::KittyOnSaled{
            owner: alice, 
            kitty_id:0,
        }.into(), );
    });
}

#[test]
fn it_sale_kitty_not_exit() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20),Error::<Test>::InvalidKittyId);
    });
}

#[test]
fn it_sale_not_kitty_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(bob),0,10,20),Error::<Test>::NotOwner);
    });
}

#[test]
fn it_sale_kitty_saled() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20));
        assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20),Error::<Test>::KittyAlreadyOnSale);
    });
}


#[test]
fn it_works_bid() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        let tom=sp_core::sr25519::Public::from_raw([2u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20));
        run_to_block(3); 
        let old_balance1=<mock::Test as pallet::Config>::Currency::free_balance(bob);
        let old_stake1=<mock::Test as pallet::Config>::Currency::reserved_balance(bob);
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(bob),0,30));
        // 检查存储项
        assert_eq!(KittiesBid::<Test>::get(0),Some((bob,30)));
        // 检查账户变化
        let stake:u128=<mock::Test as pallet::Config>::KittyStake::get();
            // 检查账户1金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::free_balance(bob),old_balance1-stake);
            // 检查账户1的stake金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::reserved_balance(bob),old_stake1+stake);
        System::assert_has_event(Event::KittyBided {
            bidder: bob, 
            kitty_id:0,
         }.into(), );
        run_to_block(4);
        let old_balance2=<mock::Test as pallet::Config>::Currency::free_balance(tom);
        let old_stake2=<mock::Test as pallet::Config>::Currency::reserved_balance(tom);
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(tom),0,50));
        // 检查存储项
        assert_eq!(KittiesBid::<Test>::get(0),Some((tom,50)));
            // 检查账户2金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::free_balance(tom),old_balance2-stake);
            // 检查账户2stake金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::reserved_balance(tom),old_stake2+stake);

         // 检查账户1金额的变化
         assert_eq!(<mock::Test as pallet::Config>::Currency::free_balance(bob),old_balance1);
         // 检查账户1stake金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::reserved_balance(bob),old_stake1);
        System::assert_has_event(Event::KittyBided {
            bidder: tom, 
            kitty_id:0,
         }.into(), );
    });
}

#[test]
fn it_bid_when_kitty_not_exit() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([2u8; 32]);
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(alice),0,50),Error::<Test>::InvalidKittyId);
    });
}


#[test]
fn it_bid_when_bidder_is_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        run_to_block(2); 
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20));
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(alice),0,50),Error::<Test>::BidderIsOwner);
        
    });
}


#[test]
fn it_bid_when_kitty_not_on_sale() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(bob),0,50),Error::<Test>::KittyNotONSale);
    });
}

#[test]
fn it_bid_price_not_high1() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20));
        run_to_block(3); 
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(bob),0,10),Error::<Test>::PriceNotHigh);
    });
}

#[test]
fn it_bid_price_not_high2() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        let tom=sp_core::sr25519::Public::from_raw([2u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20));
        run_to_block(3); 
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(bob),0,30));
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(tom),0,20),Error::<Test>::PriceNotHigh);
    });
}

#[test]
fn it_bid_not_enough_for_staking() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        run_to_block(2); 
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20));
        run_to_block(3); 
        // 将账号1的所有可用余额（不包括押金）转移到账号0
        assert_ok!(<mock::Test as pallet::Config>::Currency::transfer_all(RuntimeOrigin::signed(bob),alice,true));
        assert_noop!(KittiesModule::bid(RuntimeOrigin::signed(bob),0,30),Error::<Test>::NotEnoughForStaking);
    });
}

#[test]
fn it_trade_work() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(1);
        let alice=sp_core::sr25519::Public::from_raw([0u8; 32]);
        let bob=sp_core::sr25519::Public::from_raw([1u8; 32]);
        let tom=sp_core::sr25519::Public::from_raw([0u8; 32]);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice),100));
        let old_owner_balance0=<mock::Test as pallet::Config>::Currency::free_balance(alice);
        let old_owner_stake0=<mock::Test as pallet::Config>::Currency::reserved_balance(alice);
        let stake:u128=<mock::Test as pallet::Config>::KittyStake::get();
        run_to_block(2); 
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(alice),0,10,20));
        run_to_block(3); 
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(bob),0,30));
        run_to_block(4); 
        let old_balance2=<mock::Test as pallet::Config>::Currency::free_balance(tom);
        let old_stake2=<mock::Test as pallet::Config>::Currency::reserved_balance(tom);
        assert_ok!(KittiesModule::bid(RuntimeOrigin::signed(tom),0,100));
        run_to_block(15);
        // 检查账户变化
            // 检查账户2金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::free_balance(tom),old_balance2-stake-100);
            // 检查账户2stake金额的变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::reserved_balance(tom),old_stake2+stake);
            // 检查账户1的stake金额变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::reserved_balance(alice),old_owner_stake0-stake);
            // 检查账户1的余额变化
        assert_eq!(<mock::Test as pallet::Config>::Currency::free_balance(alice),old_owner_balance0+stake+100);
        // 检查存储项
            // owner
        assert_eq!(KittyOwner::<Test>::get(0),Some(tom));
            // sale
        assert!(!KittyOnSale::<Test>::contains_key(0));
            // bid
        assert!(!KittyOnSale::<Test>::contains_key(0));
    });
}
