//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as pallet_kitties;
use frame_benchmarking::v2::*;
use frame_support::{pallet_prelude::Get, BoundedVec};
use frame_support::sp_runtime::{Saturating,traits::One};
use frame_system::RawOrigin;
use frame_system::pallet_prelude::BlockNumberFor;
use frame_support::traits::{Currency};

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

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        create(RawOrigin::Signed(caller.clone()));

        assert_eq!(NextKittyId::<T>::get(), 1);
        assert_eq!(Kitties::<T>::get(0), Some(Kitty(Pallet::<T>::random_value(&caller))));
        assert_eq!(KittyOwner::<T>::get(0), Some(caller));

        Ok(())
    }

    #[benchmark]
    fn breed() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let kitty_id = 0;
        let another_kitty_id = 1;

        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());
        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());

        #[extrinsic_call]
        breed(RawOrigin::Signed(caller.clone()), kitty_id, another_kitty_id);

        assert_eq!(NextKittyId::<T>::get(), 3);
        assert_eq!(Kitties::<T>::get(2), Some(Kitty(Pallet::<T>::breed_kitty(&caller, Pallet::<T>::random_value(&caller), Pallet::<T>::random_value(&caller)))));
        assert_eq!(KittyOwner::<T>::get(2), Some(caller));        
        Ok(())
    }

    #[benchmark]
    fn sale() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        // define block number type is BlockNumberFor<T>
        let block_number = 10u32.into();

        // create a kitty
        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into())?;

        #[extrinsic_call]
        sale(RawOrigin::Signed(caller.clone()), 0, block_number);

        assert_eq!(KittyOnSale::<T>::get(0), Some(block_number));

        Ok(())
    }

    #[benchmark]
    fn bid() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let buyer: T::AccountId = account("buyer", 0, 0);

        let kitty_id = 0;
        let price: BalanceOf<T> = 100u32.into();
        let block_number = 10u32.into();

        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());
        Pallet::<T>::sale(RawOrigin::Signed(caller.clone()).into(), 0, block_number);

        #[extrinsic_call]
        bid(RawOrigin::Signed(buyer.clone()), kitty_id, price);
        
        assert_eq!(KittiesBid::<T>::get(kitty_id).unwrap()[0], (buyer.clone(),price));
        Ok(())
    }

    #[benchmark]
    fn transfer() -> Result<(), BenchmarkError> {
        let blocks: BlockNumberFor<T> = 11u32.into();
        let kitty_id = 0;
        let caller: T::AccountId = whitelisted_caller();

        T::Currency::make_free_balance_be(&caller, 3000u32.into());
        let buyer =  create_funded_user::<T>("buyer", 0, 1000);
        
        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());
        Pallet::<T>::sale(RawOrigin::Signed(caller.clone()).into(),kitty_id,10u32.into());
        Pallet::<T>::bid(RawOrigin::Signed(buyer.clone()).into(),kitty_id,100u32.into());
        

        let valid_till = frame_system::Pallet::<T>::block_number()
            .saturating_add(blocks)
			.saturating_add(One::one())
			.saturating_add(One::one());
        frame_system::Pallet::<T>::set_block_number(valid_till.saturating_add(One::one()));

        // balance should be enough

        let _ = T::Currency::make_free_balance_be(&buyer, 1000u32.into());

        #[extrinsic_call]
        transfer(RawOrigin::Signed(caller.clone()),kitty_id);
        
        assert_eq!(KittyOwner::<T>::get(kitty_id), Some(buyer));
        assert_eq!(KittyOnSale::<T>::get(kitty_id), None);
        assert_eq!(KittiesBid::<T>::get(kitty_id), None);
        Ok(())
    }

    impl_benchmark_test_suite!(pallet_kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
