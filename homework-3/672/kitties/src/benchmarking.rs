//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use frame_benchmarking::v2::*;
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_std::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create() {
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        create(RawOrigin::Signed(caller.clone()));
        assert!(Kitties::<T>::get(0).is_some());
    }

    #[benchmark]
    fn breed() {
        let caller: T::AccountId = whitelisted_caller();
        let _ = Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());
        let _ = Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());

        #[extrinsic_call]
        breed(RawOrigin::Signed(caller.clone()), 0, 1);
        assert!(Kitties::<T>::get(2).is_some());
    }

    #[benchmark]
    fn transfer() {
        let caller: T::AccountId = whitelisted_caller();
        let _ = Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());

        let recipient: T::AccountId = account("recipient", 0, 0);

        #[extrinsic_call]
        transfer(
            RawOrigin::Signed(caller.clone()),
            0,
            recipient.clone().into(),
        );
        assert!(Kitties::<T>::get(0).is_some());
    }

    #[benchmark]
    fn sale() {
        let caller: T::AccountId = whitelisted_caller();
        let _ = Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());

        #[extrinsic_call]
        sale(RawOrigin::Signed(caller.clone()), 0, 10u32.into());
        assert_eq!(KittyOnSale::<T>::get(0), Some(10u32.into()));
    }

    #[benchmark]
    fn bid() {
        let caller: T::AccountId = whitelisted_caller();
        let _ = Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());
        let _ = Pallet::<T>::sale(RawOrigin::Signed(caller.clone()).into(), 0, 10u32.into());
        assert_eq!(KittyOnSale::<T>::get(0), Some(10u32.into()));

        let recipient: T::AccountId = account("recipient", 0, 0);
        T::Currency::make_free_balance_be(&recipient, 20000u32.into());

        #[extrinsic_call]
        bid(RawOrigin::Signed(recipient.clone()), 0, 100u32.into());
        assert_eq!(
            KittiesBid::<T>::get(0),
            Some(vec![(recipient, 100u32.into())])
        );
    }
}
