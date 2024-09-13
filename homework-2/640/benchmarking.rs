#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use frame_support::{BoundedVec,pallet_prelude::Get};

use sp_std::vec;

#[benchmarks]
mod benches {
    use super::*;
    
    #[benchmark]
    fn create_claim(b: Linear<1,{T::MaxClaimLenth::get()}>)->Result<(),BenchmarkError>{
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();

        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()),claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller,frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }
 
    #[benchmark]
    fn revoke_claim() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();  // 获取一个白名单的账户
        let claim_length = T::MaxClaimLenth::get();  // 获取最大存证长度
        let claim = BoundedVec::try_from(vec![0; claim_length as usize]).unwrap();  // 创建声明

        // 首先手动将声明插入存储，模拟之前已经存在的声明
        Proofs::<T>::insert(&claim, (caller.clone(), frame_system::Pallet::<T>::block_number()));

        // 模拟撤销声明的 extrinsic 调用
        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        // 验证声明是否已从存储中移除
        assert_eq!(Proofs::<T>::get(&claim), None);

        Ok(())
    }

    /// 基准测试：转移存证
    #[benchmark]
    fn transfer_claim() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();  // 获取原所有者账户
        let receiver: T::AccountId = whitelisted_caller();  // 获取新的所有者账户
        let claim_length = T::MaxClaimLenth::get();  // 获取最大存证长度
        let claim = BoundedVec::try_from(vec![0; claim_length as usize]).unwrap();  // 创建声明

        // 首先手动将声明插入存储，模拟之前已经存在的声明
        Proofs::<T>::insert(&claim, (caller.clone(), frame_system::Pallet::<T>::block_number()));

        // 模拟转移声明的 extrinsic 调用
        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), receiver.clone());

        // 验证声明的所有者是否已变更为新的所有者
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((receiver.clone(), frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }
}
