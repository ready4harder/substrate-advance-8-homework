#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use frame_support::{pallet_prelude::Get, BoundedVec};
use sp_std::vec;

// 编译命令：cargo build --profile=production --features runtime-benchmarks

// 运行基准测试的命令
/**
./target/production/solochain-template-node benchmark pallet \--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet pallet_poe \
--extrinsic "*" \
--steps 20 \
--repeat 10 \
--output pallets/poe/src/weights.rs \
--template .maintain/frame-weight-template.hbs
 */

// 生产环境构建命令：cargo build --profile=production

#[benchmarks]
mod benches {
    use super::*;

    // 创建声明的基准测试
    #[benchmark]
    fn create_claim(b: Linera<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());
        // 验证声明是否正确创建
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller, frame_system::Pallet::<T>::block_number()))
        );
        Ok(())
    }

    // 撤销声明的基准测试
    #[benchmark]
    fn revoke_claim(b: Linera<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        // 首先创建一个声明
        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone())?;
        // 验证声明是否正确创建
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller.clone(), frame_system::Pallet::<T>::block_number()))
        );
        // 撤销声明
        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller.clone()), claim.clone());
        // 验证声明是否已被撤销
        assert_eq!(Proofs::<T>::get(&claim), None);

        Ok(())
    }

    // 转移声明的基准测试
    #[benchmark]
    fn transfer_claim(b: Linera<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        let to: T::AccountId = account("recipient", 0, 0);

        // 首先创建一个声明
        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone())?;
        // 验证声明是否正确创建
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller.clone(), frame_system::Pallet::<T>::block_number()))
        );
        // 转移声明
        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), to.clone());
        // 验证声明是否已被转移
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((to.clone(), frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }
}