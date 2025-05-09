#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_support::{pallet_prelude::Get, BoundedVec};
use frame_system::RawOrigin;
use sp_std::vec;

#[benchmarks]
mod benches {
    use super::*;
    #[benchmark]
    fn create_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        // 获取白名单账户
        let caller: T::AccountId = whitelisted_caller();

        // 将b 个值为0的usize变量 转换为 BoundedVec；
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

        // 外部函数调用；
        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        // 证据的KEY是否是（caller+block_number）；Proofs为全局存储映射
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller.clone(), frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }
 

    #[benchmark]
    fn revoke_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        // 外部账户1
        let caller1: T::AccountId = whitelisted_caller();


        // 将b 个值为0的usize变量 转换为 BoundedVec；
        let claim = BoundedVec::try_from(sp_std::vec![0; b as usize]).unwrap();

        // 创建证据； 
        Proofs::<T>::insert(
            &claim,
            (caller1.clone(), frame_system::Pallet::<T>::block_number()),
        );
        
        // 外部函数调用；删除证据
        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller1.clone()), claim.clone());

        // 证据已经不存在了
        assert_eq!(Proofs::<T>::get(&claim), None);

        Ok(())
    }

    #[benchmark]
    fn transfer_claim(b: Linear<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        // 获取白名单账户
        let caller1: T::AccountId = whitelisted_caller();

        // 将b 个值为0的usize变量 转换为 BoundedVec；
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

         // 创建证据； 
         Proofs::<T>::insert(
            &claim,
            (caller1.clone(), frame_system::Pallet::<T>::block_number()),
        );

       

        // 外部函数调用； transfer_claim

       
        let caller2: T::AccountId = account("recipient", 0, 0);

        
        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller1.clone()), claim.clone(), caller2.clone());  
        

        // 验证证据是否成功传输了；
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller2.clone(), frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
