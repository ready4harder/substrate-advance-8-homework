//! Benchmarking setup for pallet-kitties
#![cfg(feature = "runtime-benchmarks")]
use super::*;
#[allow(unused)]
use crate::Pallet as PalletKitties;
use frame_benchmarking::v2::*;
use frame_support::traits::{Currency, Get, ReservableCurrency};
use frame_support::{assert_ok, pallet_prelude::*};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};
use sp_std::vec;
fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    balance_factor: u32,
) -> T::AccountId {
    let user = account(string, n, 0);
    let balance = T::Currency::minimum_balance() * balance_factor.into();
    let _ = T::Currency::make_free_balance_be(&user, balance);
    user
}
fn assert_has_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}
#[benchmarks]
mod benchmarks {

    use super::*;
    #[benchmark]
    fn create() {
        let (creator, kitty_id) = (whitelisted_caller::<T::AccountId>(), 1);
        T::Currency::make_free_balance_be(&creator, 3000u32.into());
        let origin_reserved_balance = <T as Config>::Currency::reserved_balance(&creator);
        let origin_free_balance = <T as Config>::Currency::free_balance(&creator);

        #[extrinsic_call]
        create(RawOrigin::Signed(creator.clone()));

        assert_eq!(KittyId::<T>::get(), kitty_id);
        assert!(Kitties::<T>::get(kitty_id).is_some());
        assert_eq!(KittyOwner::<T>::get(kitty_id).as_ref(), Some(&creator));
        assert_eq!(
            <T as Config>::Currency::reserved_balance(&creator),
            origin_reserved_balance + <T as Config>::StakeAmount::get()
        );
        assert_eq!(
            <T as Config>::Currency::free_balance(&creator),
            origin_free_balance - <T as Config>::StakeAmount::get()
        );
        assert_has_event::<T>(
            Event::<T>::KittyCreated {
                creator,
                kitty_id,
                data: Kitties::<T>::get(kitty_id).unwrap().0.clone(),
            }
                .into(),
        );
    }

    #[benchmark]
    fn breed() {
        let (creator, kitty_id_1, kitty_id_2, kitty_id) =
            (whitelisted_caller::<T::AccountId>(), 1, 2, 3);
        T::Currency::make_free_balance_be(&creator, 20000u32.into());

        assert_ok!(Pallet::<T>::create(
            RawOrigin::Signed(creator.clone()).into()
        ));
        assert_ok!(Pallet::<T>::create(
            RawOrigin::Signed(creator.clone()).into()
        ));
        let origin_reserved_balance = <T as Config>::Currency::reserved_balance(&creator);
        let origin_free_balance = <T as Config>::Currency::free_balance(&creator);

        #[extrinsic_call]
        breed(RawOrigin::Signed(creator.clone()), kitty_id_1, kitty_id_2);

        assert_eq!(KittyId::<T>::get(), kitty_id);
        assert_eq!(
            <T as Config>::Currency::reserved_balance(&creator),
            origin_reserved_balance + <T as Config>::StakeAmount::get()
        );
        assert_eq!(
            <T as Config>::Currency::free_balance(&creator),
            origin_free_balance - <T as Config>::StakeAmount::get()
        );
        assert_has_event::<T>(
            Event::<T>::KittyCreated {
                creator,
                kitty_id,
                data: Kitties::<T>::get(kitty_id).unwrap().0.clone(),
            }
                .into(),
        );
    }

    #[benchmark]
    fn transfer() {
        let (from, to, kitty_id) = (
            whitelisted_caller::<T::AccountId>(),
            create_funded_user::<T>("to", 0, 1000),
            1,
        );
        T::Currency::make_free_balance_be(&from, 3000u32.into());

        assert_ok!(Pallet::<T>::create(RawOrigin::Signed(from.clone()).into()));

        let origin_reserved_balance_of_from = <T as Config>::Currency::reserved_balance(&from);
        let origin_reserved_balance_of_to = <T as Config>::Currency::reserved_balance(&to);
        let origin_free_balance_of_from = <T as Config>::Currency::free_balance(&from);
        let origin_free_balance_of_to = <T as Config>::Currency::free_balance(&to);
        #[extrinsic_call]
        transfer(RawOrigin::Signed(from.clone()), to.clone(), kitty_id);

        assert_eq!(KittyOwner::<T>::get(kitty_id).as_ref(), Some(&to));
        let stake_amount = <T as Config>::StakeAmount::get();
        assert_eq!(
            <T as Config>::Currency::reserved_balance(&from),
            origin_reserved_balance_of_from - stake_amount
        );
        assert_eq!(
            <T as Config>::Currency::reserved_balance(&to),
            origin_reserved_balance_of_to + stake_amount
        );
        assert_eq!(
            <T as Config>::Currency::free_balance(&from),
            origin_free_balance_of_from + stake_amount
        );
        assert_eq!(
            <T as Config>::Currency::free_balance(&to),
            origin_free_balance_of_to - stake_amount
        );
        assert_has_event::<T>(Event::<T>::KittyTransferred { from, to, kitty_id }.into());
    }

    #[benchmark]
    fn sale() {
        let caller: T::AccountId = whitelisted_caller();
        let kitty_id = 1;
        Kitties::<T>::insert(kitty_id, Kitty([0_u8;16]));
        KittyOwner::<T>::insert(kitty_id, &caller);

        let until_block = 100u32.into();
        #[extrinsic_call]
        sale(RawOrigin::Signed(caller.clone()), kitty_id, until_block);

        assert_eq!(
            KittiesOnSale::<T>::get(&until_block),
            BoundedVec::<u32, <T as Config>::MaxKittiesBidPerBlock>::try_from(vec![kitty_id])
                .unwrap()
        );
        assert!(KittiesBid::<T>::contains_key(&kitty_id));
        assert_has_event::<T>(
            Event::<T>::KittyOnSale {
                owner : caller,
                kitty_id,
                until_block,
            }
                .into(),
        );
    }

    #[benchmark]
    fn bid() {
        let until_block: BlockNumberFor<T> = 10u32.into();
        let balance_price: BalanceOf<T> = 1000u32.into();
        let (owner, bidder, kitty_id, price, until_block) = (
            whitelisted_caller::<T::AccountId>(),
            create_funded_user::<T>("bidder", 0, 1000),
            1,
            balance_price,
            until_block,
        );
        T::Currency::make_free_balance_be(&owner, 3000u32.into());

        Kitties::<T>::insert(kitty_id, Kitty([0_u8;16]));
        KittyOwner::<T>::insert(kitty_id, &owner);

        let _ =KittiesOnSale::<T>::try_append(&until_block, kitty_id);
        KittiesBid::<T>::insert(kitty_id, Some((owner.clone(), T::MinBidIncrement::get())));

        #[extrinsic_call]
        bid(RawOrigin::Signed(bidder.clone()), kitty_id, price);

        assert_eq!(
            KittiesBid::<T>::get(kitty_id),
            Some((bidder.clone(), price))
        );
        assert_has_event::<T>(
            Event::<T>::KittyBid {
                bidder,
                kitty_id,
                price,
            }
                .into(),
        );
    }

    impl_benchmark_test_suite!(
        PalletKitties,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );

}
