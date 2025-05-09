// 添加编译标签
#![cfg(feature="runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
// 表示交易的发送方
use frame_system::RawOrigin;
use frame_support::{BoundedVec,pallet_prelude::Get};
use sp_std::vec;

#[benchmarks]
mod benches{
    use super::*;
    // 标志位基准测试函数
    #[benchmark]
    // b表示测试输入的大小
    fn create_claim(b: Linear<1,{T::MaxClaimLength::get()}>)->Result<(),BenchmarkError>{
        // 获取账户id
        let caller: T::AccountId=whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0;b as usize]).unwrap();

        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()),claim.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller,frame_system::Pallet::<T>::block_number()))
        );
        Ok(())
    }
    // use super::*;
    #[benchmark]
    fn revoke_claim(b:Linear<1,{T::MaxClaimLength::get()}>)->Result<(),BenchmarkError>{
        let caller: T::AccountId=whitelisted_caller();
        let claim =BoundedVec::try_from(vec![0;b as usize]).unwrap();
        // 先创建一个存证
        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(),claim.clone());
        #[extrinsic_call]
        // 撤销存证
        revoke_claim(RawOrigin::Signed(caller.clone()),claim.clone());
        assert_eq!(
            Proofs::<T>::get(&claim),
            None
        );
        Ok(())
    }

    #[benchmark]
    fn transfer_claim(b:Linear<1,{T::MaxClaimLength::get()}>)->Result<(),BenchmarkError>{
        let caller: T::AccountId=whitelisted_caller();
        let claim =BoundedVec::try_from(vec![0;b as usize]).unwrap();
        // 创建一个接受方的账户
        let recipent: T::AccountId=account("recipent",0,0);
        // #[extrinsic_call]
        // 先创建一个存证，创建方为caller
        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(),claim.clone());
        #[extrinsic_call]
        // 转移存证
        // transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), recipent);
        transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), recipent.clone());

        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((recipent,frame_system::Pallet::<T>::block_number()))
        );
        Ok(())
    }

}