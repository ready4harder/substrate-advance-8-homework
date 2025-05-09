//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};
use frame_support::traits::Currency;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, 3000u32.into());

        #[extrinsic_call]   
        crate::create(RawOrigin::Signed(caller));

        assert_eq!(Kitties::<T>::contains_key(0), true);

        Ok(())
    }    

    #[benchmark]
    fn breed() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        <T as pallet::Config>::Currency::make_free_balance_be(&caller,6000u32.into());
        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into())?;
        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into())?;

        assert_eq!(Kitties::<T>::contains_key(0), true);
        assert_eq!(Kitties::<T>::contains_key(1), true);

        #[extrinsic_call]
        crate::breed(RawOrigin::Signed(caller), 0, 1);

        assert_eq!(Kitties::<T>::contains_key(2), true);
        Ok(())
    }    
 
    #[benchmark]
    fn transfer() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, 3000u32.into());
        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into())?;
        assert_eq!(Kitties::<T>::contains_key(0), true);
        assert_eq!(KittyOwner::<T>::contains_key(0), true);
        assert_eq!(KittyOwner::<T>::get(0).unwrap(), caller);

        let receiver: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&receiver, 3000u32.into());
        #[extrinsic_call]
        crate::transfer(RawOrigin::Signed(caller), receiver.clone(), 0);

        assert_eq!(KittyOwner::<T>::get(0).unwrap(), receiver);

        Ok(())
    }    

    #[benchmark]
    fn sale() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, 3000u32.into());
        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into())?;
        assert_eq!(Kitties::<T>::contains_key(0), true);

        let price: BalanceOf<T> = 1000u32.into();
        let to_block: BlockNumberFor<T> = 5u32.into();
        #[extrinsic_call]
        crate::sale(RawOrigin::Signed(caller), 0, price, to_block);

        assert_eq!(KittiesBid::<T>::contains_key(0), true);
        assert_eq!(KittiesBid::<T>::get(0).unwrap().is_empty(), true);
        assert_eq!(KittiesSaleInfo::<T>::contains_key(0), true);

        Ok(())
    }    
   
    #[benchmark]
    fn bid() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, 3000u32.into());
        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into())?;
        let price: BalanceOf<T> = 1000u32.into();
        let to_block: BlockNumberFor<T> = 5u32.into();
        Pallet::<T>::sale(RawOrigin::Signed(caller.clone()).into(), 0, price, to_block)?;

        let bidder: T::AccountId = account("bidder", 0, 0);
        T::Currency::make_free_balance_be(&bidder, 3000u32.into());

        let price: BalanceOf<T> = 1200u32.into();
        #[extrinsic_call]
        crate::bid(RawOrigin::Signed(bidder), 0, price);

        assert_eq!(KittiesBid::<T>::contains_key(0), true);
        assert_eq!(KittiesBid::<T>::get(0).unwrap().len(), 1);

        Ok(())
    }    

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
