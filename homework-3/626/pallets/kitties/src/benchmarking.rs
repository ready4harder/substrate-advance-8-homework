#![cfg(feature="runtime-benchmarks")]

use super::*;
#[allow(unused)]
use crate::Pallet as KittiesModule;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::{BoundedVec,pallet_prelude::*,assert_ok};
use frame_support::traits::Currency;
use frame_support::traits::ReservableCurrency;
use sp_std::vec;

const SEED: u32 = 0;

#[benchmarks]
mod benchmarks{
    use super::*;
    #[benchmark]
    fn create(){
        let caller:T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        create(RawOrigin::Signed(caller.clone()));
        assert_eq!(KittyOwner::<T>::get(0),Some(caller.clone()));
        assert_eq!(NextKittyId::<T>::get(),1);
        assert_eq!(Kitties::<T>::get(0).is_some(),true);
    }

    #[benchmark]
    fn breed(){
        let caller:T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        breed(RawOrigin::Signed(caller.clone()),0,1);
    }


    #[benchmark]
    fn transfer(){
        let caller1:T::AccountId = whitelisted_caller();
        let caller2: T::AccountId = account("recipient", 0, SEED);
        KittiesModule::<T>::create(RawOrigin::Signed(caller1.clone()).into());
        #[extrinsic_call]
        transfer(RawOrigin::Signed(caller1.clone()),caller2.clone(),0);

    }

    #[benchmark]
    fn sale(){
        let caller:T::AccountId = whitelisted_caller();
        KittiesModule::<T>::create(RawOrigin::Signed(caller.clone()).into());
        #[extrinsic_call]
        sale(RawOrigin::Signed(caller.clone()),0,10u32.into(), 20u32.into());
    }

    #[benchmark]
    fn bid(){
        let caller1:T::AccountId = whitelisted_caller();
        let caller2: T::AccountId = account("bidder", 0, SEED);
        KittiesModule::<T>::create(RawOrigin::Signed(caller1.clone()).into());
        KittiesModule::<T>::sale(RawOrigin::Signed(caller1.clone()).into(),0,10u32.into(), 20u32.into());
        #[extrinsic_call]
        bid(RawOrigin::Signed(caller2.clone()),0,30u32.into());

    }
}