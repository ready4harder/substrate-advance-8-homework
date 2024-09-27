use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use frame_system::Config;
use crate::{NextKittyId,KittyOwner,KittiesOnSale,KittiesBid};
use frame_support::traits::Currency;

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
        let alice: u64 = 0;
        let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller));

        System::assert_has_event(
            Event::KittyCreated {
                creator: alice, 
                index: 0, 
                data:[0_u8;16], 
            }
            .into(),
        );
        
    });

}

#[test]
fn it_kitty_id_overflow(){
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice : u64 = 0;
        let caller :RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);
        NextKittyId::<Test>::put(u32::MAX);
        assert_noop!(Kitties::create(caller), Error::<Test>::KittyIdOverflow);

    });
        


}

#[test]
    fn it_breeds_kitties() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);

            // Create two kitties first
            assert_ok!(Kitties::create(caller.clone()));
            assert_ok!(Kitties::create(caller.clone()));

            // Breed the two kitties
            assert_ok!(Kitties::breed(caller.clone(), 0, 1));

        });
    }

#[test]
    fn it_fails_for_same_kitty_id() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);

            // Create a kitty first
            assert_ok!(Kitties::create(caller.clone()));

            // Attempt to breed the same kitty
            assert_noop!(
                Kitties::breed(caller.clone(), 0, 0),
                Error::<Test>::SameKittyId
            );
        });
    }

#[test]
    fn it_fails_for_non_existent_kitty() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);

            // Attempt to breed non-existent kitties
            assert_noop!(
                Kitties::breed(caller.clone(), 0, 1),
                Error::<Test>::KittyNotFound
            );
        });
    }

 #[test]
    fn it_works_transfer_kitty() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let bob: u64 = 1;
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);
    
            // Create a kitty for Alice
            assert_ok!(Kitties::create(caller.clone()));

            // Verify that Alice is the owner of the kitty
            assert_eq!(KittyOwner::<Test>::get(0), Some(alice));
    
            // Transfer the kitty from Alice to Bob
            assert_ok!(Kitties::transfer(caller, bob, 0));
    
            // Check that the transfer event was emitted
            System::assert_has_event(
                Event::KittyTransferred {
                    from: alice,
                    to: bob,
                    kitty_id: 0,
                }
                .into(),
            );
    
            // Verify that Bob is now the owner of the kitty
            assert_eq!(KittyOwner::<Test>::get(0), Some(bob));
        });
    }
    
#[test]
    fn it_fails_transfer_kitty_not_found() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let bob: u64 = 1;
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);
    
            // Attempt to transfer a non-existent kitty
            assert_noop!(Kitties::transfer(caller, bob, 0), Error::<Test>::KittyNotFound);
        });
    }
    
#[test]
    fn it_fails_transfer_not_owner() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let bob: u64 = 1;
            let charlie: u64 = 2;
            let alice_caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);
            let bob_caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(bob);
    
            // Create a kitty for Alice
            assert_ok!(Kitties::create(alice_caller.clone()));
    
            // Attempt to transfer the kitty from Bob (who is not the owner)
            assert_noop!(Kitties::transfer(bob_caller, charlie, 0), Error::<Test>::NotOwner);
        });
    }


#[test]
    fn it_works_sale() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let alice_caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);
            
            // Create a kitty for Alice
            assert_ok!(Kitties::create(alice_caller.clone()));
            
            // Put the kitty on sale
            assert_ok!(Kitties::sale(alice_caller, 0, 10));
            
            // Check if the kitty is on sale
            assert_eq!(KittiesOnSale::<Test>::get(0), Some((alice, 10)));
        });
    }

 #[test]
    fn it_failed_kitty_already_on_sale() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let alice_caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);
            
            // Create a kitty for Alice
            assert_ok!(Kitties::create(alice_caller.clone()));
            
            // Put the kitty on sale
            assert_ok!(Kitties::sale(alice_caller.clone(), 0, 10));
            
            
            // Attempt to put the same kitty on sale again
            assert_noop!(Kitties::sale(alice_caller, 0, 20), Error::<Test>::KittyAlreadyOnSale);
        });
    }
    
#[test]
    fn it_failed_sale_not_owner() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let bob: u64 = 1;
            let alice_caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(alice);
            let bob_caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(bob);
            
            // Create a kitty for Alice
            assert_ok!(Kitties::create(alice_caller.clone()));
    
            // Attempt to put the kitty on sale by Bob (who is not the owner)
            assert_noop!(Kitties::sale(bob_caller, 0, 10), Error::<Test>::NotOwner);
        });
    }
    
    #[test]
    fn it_fails_bid_kitty_not_found() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let bob: u64 = 1;
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(bob);
            let new_bid = 150u64;
    
            // Try to place a bid on a non-existent kitty
            assert_noop!(
                Kitties::bid(caller.clone(), 999, new_bid), // Use a non-existent kitty ID
                Error::<Test>::KittyNotFound
            );
        });
    }
    
    #[test]
    fn it_fails_bid_kitty_not_for_sale() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let bob: u64 = 1;
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(bob);
    
            // Create a kitty
            assert_ok!(Kitties::create(<<Test as Config>::RuntimeOrigin>::signed(alice)));
    
            // Try to place a bid on a kitty not for sale
            assert_noop!(
                Kitties::bid(caller.clone(), 0, 150u64),
                Error::<Test>::KittyNotForSale
            );
        });
    }
    

    #[test]
    fn it_fails_bid_too_low() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let bob: u64 = 1;
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(bob);
            let price = 100u64;
            let low_bid = 50u64;
            let high_bid = 150u64;
    
            // Set bob's balance to be sufficient for the high bid
            Balances::deposit_creating(&bob, 200);
    
            // Create a kitty and put it on sale
            assert_ok!(Kitties::create(<<Test as Config>::RuntimeOrigin>::signed(alice)));
            assert_ok!(Kitties::sale(<<Test as Config>::RuntimeOrigin>::signed(alice), 0, price));
    
            // Place a high bid
            assert_ok!(Kitties::bid(caller.clone(), 0, high_bid));
    
            // Check the current highest bid before trying a low bid
            let current_bids = KittiesBid::<Test>::get(0);
            assert_eq!(current_bids.unwrap().last().unwrap().1, high_bid);
    
            // Try to place a low bid
            assert_noop!(
                Kitties::bid(caller.clone(), 0, low_bid),
                Error::<Test>::BidTooLow
            );
        });
    }
    
    #[test]
    fn it_works_bid() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0;
            let bob: u64 = 1;
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(bob);
            let price = 100u64;
            let new_bid = 150u64;
    
            // Set bob's balance to be sufficient for the bid
            Balances::deposit_creating(&bob, 200);
    
            // Create a kitty and put it on sale
            assert_ok!(Kitties::create(<<Test as Config>::RuntimeOrigin>::signed(alice)));
            assert_ok!(Kitties::sale(<<Test as Config>::RuntimeOrigin>::signed(alice), 0, price));
    
            // Place a bid
            assert_ok!(Kitties::bid(caller.clone(), 0, new_bid));
        });
    }

    #[test]
    fn it_fails_bid_insufficient_balance() {
        new_test_ext().execute_with(|| {
            run_to_block(1);
            let alice: u64 = 0; // Alice's account ID
            let bob: u64 = 1;   // Bob's account ID
            let caller: RuntimeOrigin = <<Test as Config>::RuntimeOrigin>::signed(bob);
            let price = 100u64; // Sale price of the kitty
            let new_bid = 150u64; // Bob's attempted bid amount
    
            // Step 1: Create a kitty and put it on sale
            assert_ok!(Kitties::create(<<Test as Config>::RuntimeOrigin>::signed(alice)));
            assert_ok!(Kitties::sale(<<Test as Config>::RuntimeOrigin>::signed(alice), 0, price));
    
            // Step 2: Set Bob's balance to be insufficient for the bid
            Balances::deposit_creating(&bob, 100); // Bob has only 100 units
    
            // Step 3: Check Bob's balance before the bid attempt
            assert_eq!(Balances::free_balance(&bob), 100);
    
            // Step 4: Attempt to place a bid that exceeds Bob's balance
            assert_noop!(
                Kitties::bid(caller.clone(), 0, new_bid),
                pallet_balances::Error::<Test>::InsufficientBalance // Expecting the InsufficientBalance error
            );
    
            // Step 5: Ensure Bob's balance remains unchanged after the failed bid
            assert_eq!(Balances::free_balance(&bob), 100);
        });
    }
