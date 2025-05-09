use crate::{mock::*, Error, Event};
use frame_support::assert_ok;
use frame_support::traits::Currency;
use frame_system::Config;

use codec::Decode;
use sp_core::offchain::{testing, OffchainWorkerExt, TransactionPoolExt};
use sp_keystore::{testing::MemoryKeystore, Keystore, KeystoreExt};
use sp_runtime::RuntimeAppPublic;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

#[test]
fn it_works_for_create_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = test_pub(1);
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);

        assert_ok!(Kitties::create(caller.clone()));
        assert_eq!(Kitties::kitties_owner(0), Some(alice));
    });
}

#[test]
fn it_works_for_breed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = test_pub(1);
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller.clone()));
        System::inc_account_nonce(&alice);
        assert_ok!(Kitties::create(caller.clone()));
        match Kitties::breed(caller.clone(), 0, 1) {
            Ok(()) => {
                // breed successfully
                assert_eq!(Kitties::kitties_owner(2), Some(alice));
            }
            Err(e) => {
                // breed failed, because same gender
                assert_eq!(e, Error::<Test>::SameGender.into());
            }
        }
    });
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice = test_pub(1);
        let bob = test_pub(2);
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller.clone()));

        assert_ok!(Kitties::transfer(caller.clone(), 0, bob));
        assert_eq!(Kitties::kitties_owner(0), Some(bob));
        System::assert_has_event(
            Event::KittyTransferred {
                from: alice,
                to: bob,
                index: 0,
            }
            .into(),
        );
    });
}

#[test]
fn it_works_for_sale() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let sale_account = test_pub(1);
        let bid_account = test_pub(2);
        let bid_account2 = test_pub(3);
        Balances::make_free_balance_be(&bid_account, 200);
        Balances::make_free_balance_be(&bid_account2, 200);
        assert_ok!(Kitties::create(<<Test as Config>::RuntimeOrigin>::signed(
            sale_account
        )));
        assert_ok!(Kitties::sale(
            <<Test as Config>::RuntimeOrigin>::signed(sale_account),
            0,
            10
        ));
        assert_eq!(Kitties::kitty_on_sale(0), Some(10));
        run_to_block(2);
        assert_ok!(Kitties::bid(
            <<Test as Config>::RuntimeOrigin>::signed(bid_account),
            0,
            100
        ));
        assert_ok!(Kitties::bid(
            <<Test as Config>::RuntimeOrigin>::signed(bid_account2),
            0,
            130
        ));
        run_to_block(10);

        assert_eq!(Kitties::kitty_on_sale(0), None);
        assert_eq!(Kitties::kitties_owner(0), Some(bid_account2));
    });
}

fn test_pub(dummy: u8) -> sp_core::sr25519::Public {
    sp_core::sr25519::Public::from_raw([dummy; 32])
}

#[test]
fn it_aggregates_the_price() {
    new_test_ext().execute_with(|| {
        assert_eq!(Kitties::average_price(), None);

        assert_ok!(Kitties::submit_price(
            RuntimeOrigin::signed(test_pub(1)),
            27
        ));
        assert_eq!(Kitties::average_price(), Some(27));

        assert_ok!(Kitties::submit_price(
            RuntimeOrigin::signed(test_pub(1)),
            43
        ));
        assert_eq!(Kitties::average_price(), Some(35));
    });
}

#[test]
fn should_make_http_call_and_parse_result() {
    let (offchain, state) = testing::TestOffchainExt::new();
    let mut t = sp_io::TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain));

    price_oracle_response(&mut state.write());

    t.execute_with(|| {
        // when
        let price = Kitties::fetch_price().unwrap();
        // then
        assert_eq!(price, 666);
    });
}

#[test]
fn should_submit_signed_transaction_on_chain() {
    const PHRASE: &str =
        "news slush supreme milk chapter athlete soap sausage put clutch what kitten";

    let (offchain, offchain_state) = testing::TestOffchainExt::new();
    let (pool, pool_state) = testing::TestTransactionPoolExt::new();
    let keystore = MemoryKeystore::new();
    keystore
        .sr25519_generate_new(crate::Public::ID, Some(&format!("{}/hunter1", PHRASE)))
        .unwrap();

    let mut t = sp_io::TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain));
    t.register_extension(TransactionPoolExt::new(pool));
    t.register_extension(KeystoreExt::new(keystore));

    price_oracle_response(&mut offchain_state.write());

    t.execute_with(|| {
        // when
        Kitties::fetch_price_and_send_signed().unwrap();
        // then
        let tx = pool_state.write().transactions.pop().unwrap();
        assert!(pool_state.read().transactions.is_empty());
        let tx = crate::mock::Extrinsic::decode(&mut &*tx).unwrap();
        assert_eq!(tx.signature.unwrap().0, 0);
        assert_eq!(
            tx.call,
            RuntimeCall::Kitties(crate::Call::submit_price { price: 666 })
        );
    });
}

fn price_oracle_response(state: &mut testing::OffchainState) {
    state.expect_request(testing::PendingRequest {
        method: "GET".into(),
        uri: "https://min-api.cryptocompare.com/data/price?fsym=DOT&tsyms=USD".into(),
        response: Some(br#"{"USD": 6.66}"#.to_vec()),
        sent: true,
        ..Default::default()
    });
}
