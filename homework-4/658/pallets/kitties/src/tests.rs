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
        let creator = 1;
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(1)));

        assert!(Kitties::<Test>::get(kitty_id).is_some());
        assert_eq!(PalletKitties::kitty_owner(kitty_id), Some(1));
        assert_eq!(PalletKitties::kitty_id(), 1);


        // assert_eq!(
        //     PalletKitties::owner_kitties(1),
        //     BoundedVec::<u32,  <Test as Config>::MaxKittiesOwned>::try_from(vec![kitty_id]).unwrap()
        // );
        assert_eq!(PalletKitties::owner_kitties(2), vec![]);
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
        let creator = 1;
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
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(1)));
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(1)));

        assert_eq!(KittyOwner::<Test>::get(1), Some(1));
        assert_eq!(KittyOwner::<Test>::get(2), Some(1));

        assert_ok!(PalletKitties::breed(RuntimeOrigin::signed(1), 1, 2));

        assert_eq!(KittyOwner::<Test>::get(3), Some(1));
    })
}
#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        let (from, to, kitty_id) = (1, 2, 1);
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
fn it_aggregates_the_price() {
    sp_io::TestExternalities::default().execute_with(|| {
        PalletKitties::fetch_price_and_send_raw_unsigned(1).unwrap();
        let (pool, pool_state) = testing::TestTransactionPoolExt::new();
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