use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use super::*;
use sp_keystore::{testing::MemoryKeystore, Keystore, KeystoreExt};
use sp_core::{
    offchain::{testing, OffchainWorkerExt, TransactionPoolExt},
    sr25519::Signature,
    H256,
};
use sp_runtime::{
    testing::TestXt,
    traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
    RuntimeAppPublic,
};

type Extrinsic = TestXt<RuntimeCall, ()>;
fn test_pub() -> sp_core::sr25519::Public {
    sp_core::sr25519::Public::from_raw([1u8; 32])
}

fn alice() -> sp_core::sr25519::Public {
    sp_core::sr25519::Public::from_raw([1u8; 32])
}
fn bob() -> sp_core::sr25519::Public {
    sp_core::sr25519::Public::from_raw([2u8; 32])
}
#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

#[test]
fn create_kitty() {
    new_test_ext().execute_with(|| {
        let kitty_id = 1;
        let creator = alice();

        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(creator)));

        assert!(Kitties::<Test>::get(kitty_id).is_some());
        assert_eq!(PalletKitties::kitty_owner(kitty_id), Some(creator));
        assert_eq!(PalletKitties::kitty_id(), 1);


        // assert_eq!(
        //     PalletKitties::owner_kitties(1),
        //     BoundedVec::<u32,  <Test as Config>::MaxKittiesOwned>::try_from(vec![kitty_id]).unwrap()
        // );
        // assert_eq!(PalletKitties::owner_kitties(2), vec![]);
        System::assert_has_event(
            Event::KittyCreated{
                creator,
                kitty_id,
                data: PalletKitties::kitties(kitty_id).unwrap().0,
            }
            .into()
        );
    });
}
#[test]
fn create_failed_when_next_kitty_id_overflow() {
    new_test_ext().execute_with(|| {
        let creator = alice();
        KittyId::<Test>::put(u32::MAX);
        assert_noop!(
            PalletKitties::create(RuntimeOrigin::signed(creator)),
            Error::<Test>::NextKittyIdOverflow
        );
    });
}

#[test]
fn test_breed() {
    new_test_ext().execute_with(|| {
        let breeder = alice();
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(breeder)));
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(breeder)));

        assert_eq!(KittyOwner::<Test>::get(1), Some(breeder));
        assert_eq!(KittyOwner::<Test>::get(2), Some(breeder));

        assert_ok!(PalletKitties::breed(RuntimeOrigin::signed(breeder), 1, 2));

        assert_eq!(KittyOwner::<Test>::get(3), Some(breeder));
    })
}
#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        let (from, to, kitty_id) = (alice(), bob(), 1);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(from)));
        assert_ok!(PalletKitties::transfer(
            RuntimeOrigin::signed(from),
            to,
            kitty_id
        ));
        assert_eq!(KittyOwner::<Test>::get(kitty_id), Some(to));

        System::assert_has_event(Event::<Test>::KittyTransferred { from, to, kitty_id }.into());
    });
}

#[test]
fn should_submit_raw_unsigned_transaction_on_chain() {
    let (offchain, offchain_state) = testing::TestOffchainExt::new();
    let (pool, pool_state) = testing::TestTransactionPoolExt::new();

    let keystore = MemoryKeystore::new();

    let mut t = sp_io::TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain));
    t.register_extension(TransactionPoolExt::new(pool));
    t.register_extension(KeystoreExt::new(keystore));

    price_oracle_response(&mut offchain_state.write());

    t.execute_with(|| {
        // when
        PalletKitties::fetch_price_and_send_raw_unsigned(1).unwrap();
        // then
        let tx = pool_state.write().transactions.pop().unwrap();
        assert!(pool_state.read().transactions.is_empty());
        let tx = Extrinsic::decode(&mut &*tx).unwrap();
        assert_eq!(tx.signature, None);
        assert_eq!(
            tx.call,
            RuntimeCall::PalletKitties(crate::Call::submit_price_unsigned {
                block_number: 1,
                price: 15523
            })
        );
    });
}

pub fn price_oracle_response(state: &mut testing::OffchainState) {
    state.expect_request(testing::PendingRequest {
        method: "GET".into(),
        uri: "https://min-api.cryptocompare.com/data/price?fsym=DOT&tsyms=USD".into(),
        response: Some(br#"{"USD": 155.23}"#.to_vec()),
        sent: true,
        ..Default::default()
    });
}