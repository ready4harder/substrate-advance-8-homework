#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::{pallet_prelude::*, BoundedVec, traits::Get};
use sp_std::vec;

#[benchmarks]
mod bench {
    use super::*;

    // 测试创建存证
    #[benchmark]
    fn create_claim_benchmark(c: Linear<1, { T::MaxClaimLength::get() }>) {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; c as usize]).unwrap();

        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller, frame_system::Pallet::<T>::block_number()))
        );
    }

    // 测试撤销存证
    #[benchmark]
    fn revoke_claim(c: Linear<1, { T::MaxClaimLength::get() }>) {
        let claim = BoundedVec::try_from(vec![0; c as usize]).unwrap();
        let caller: T::AccountId = whitelisted_caller();

        // 设置前置条件
        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone());

        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert!(!Proofs::<T>::contains_key(&claim));
    }

    // 测试转移存证
    #[benchmark]
    fn transfer_claim(c: Linear<1, { T::MaxClaimLength::get() }>) {
        let claim = BoundedVec::try_from(vec![0; c as usize]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        // 创建一个接受者
        let recipient: T::AccountId = account("recipient", 0, 100);

        // 设置前置条件
        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).unwrap();

        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), recipient.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((recipient, frame_system::Pallet::<T>::block_number()))
        );
    }
}