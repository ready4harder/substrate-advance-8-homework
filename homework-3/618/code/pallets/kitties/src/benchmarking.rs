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
        T::Currency::make_free_balance_be(&caller, 1000u32.into());
        let old_balance0=T::Currency::free_balance(&caller);
        let old_stake0=T::Currency::reserved_balance(&caller);
        #[extrinsic_call]
        create(RawOrigin::Signed(caller.clone()));
        // 断言
        // 存储项
            // owner
        assert_eq!(KittyOwner::<T>::get(0),Some(caller.clone()));
            // next
        assert_eq!(NextKittyId::<T>::get(),1);
            // kitty
        assert_eq!(Kitties::<T>::get(0).is_some(),true);
        // 账户金额变化
        let stake=T::KittyStake::get();
            // 检查账户金额的变化
        assert_eq!(T::Currency::free_balance(&caller),old_balance0-stake);
            // 检查stake金额的变化
        assert_eq!(T::Currency::reserved_balance(&caller),old_stake0+stake);
    }

    #[benchmark]
    fn breed(){
        let caller:T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, 3000u32.into());
        assert_ok!(KittiesModule::<T>::create(RawOrigin::Signed(caller.clone()).into()));
        assert_ok!(KittiesModule::<T>::create(RawOrigin::Signed(caller.clone()).into()));
        let old_balance0=T::Currency::free_balance(&caller);
        let old_stake0=T::Currency::reserved_balance(&caller);
        #[extrinsic_call]
        breed(RawOrigin::Signed(caller.clone()),0,1);
        // 断言
        // 存储项
            // owner
        assert_eq!(KittyOwner::<T>::get(2),Some(caller.clone()));
            // next
        assert_eq!(NextKittyId::<T>::get(),3);
            // kitty
        assert_eq!(Kitties::<T>::get(2).is_some(),true);
        // 账户金额变化
        let stake=T::KittyStake::get();
            // 检查账户金额的变化
        assert_eq!(T::Currency::free_balance(&caller),old_balance0-stake);
            // 检查stake金额的变化
        assert_eq!(T::Currency::reserved_balance(&caller),old_stake0+stake);
    }

    
    #[benchmark]
    fn transfer(){
        let caller1:T::AccountId = whitelisted_caller();
        // let caller2:T::AccountId = whitelisted_caller();
        let caller2: T::AccountId = account("recipient", 0, SEED);
        T::Currency::make_free_balance_be(&caller1, 1000u32.into());
        T::Currency::make_free_balance_be(&caller2, 1000u32.into());
        KittiesModule::<T>::create(RawOrigin::Signed(caller1.clone()).into());
        let old_balance0=T::Currency::free_balance(&caller2);
        let old_stake0=T::Currency::reserved_balance(&caller2);
        #[extrinsic_call]
        transfer(RawOrigin::Signed(caller1.clone()),0,caller2.clone());
        // 断言
        // 存储项
            // owner
        assert!(caller1 != caller2);
        assert_eq!(KittyOwner::<T>::get(0),Some(caller2.clone()));
            // kitty
        assert_eq!(Kitties::<T>::get(0).is_some(),true);
        // 账户金额变化
        let stake=T::KittyStake::get();
            // 检查账户金额的变化
        assert_eq!(T::Currency::free_balance(&caller2),old_balance0-stake);
            // 检查stake金额的变化
        assert_eq!(T::Currency::reserved_balance(&caller2),old_stake0+stake);
    }

    #[benchmark]
    fn sale(){
        let caller:T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, 1000u32.into());
        KittiesModule::<T>::create(RawOrigin::Signed(caller.clone()).into());
        #[extrinsic_call]
        sale(RawOrigin::Signed(caller.clone()),0,10u32.into(), 20u32.into());
        assert_eq!(KittyOnSale::<T>::get(0),Some((10u32.into(), 20u32.into())));
    }

    #[benchmark]
    fn bid(){
        let caller1:T::AccountId = whitelisted_caller();
        let caller2: T::AccountId = account("bidder", 0, SEED);
        T::Currency::make_free_balance_be(&caller1, 1000u32.into());
        T::Currency::make_free_balance_be(&caller2, 1000u32.into());
        KittiesModule::<T>::create(RawOrigin::Signed(caller1.clone()).into());
        KittiesModule::<T>::sale(RawOrigin::Signed(caller1.clone()).into(),0,10u32.into(), 20u32.into());
        let old_balance=T::Currency::free_balance(&caller2);
        let old_stake=T::Currency::reserved_balance(&caller2);
        #[extrinsic_call]
        bid(RawOrigin::Signed(caller2.clone()),0,30u32.into());
        // 存储项
        assert_eq!(KittiesBid::<T>::get(0),Some((caller2.clone(),30u32.into())));
        // 账户金额变化
        let stake=T::KittyStake::get();
            // 检查账户金额的变化
        assert_eq!(T::Currency::free_balance(&caller2),old_balance-stake);
            // 检查stake金额的变化
        assert_eq!(T::Currency::reserved_balance(&caller2),old_stake+stake);
    }
}