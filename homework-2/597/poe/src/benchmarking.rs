#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use frame_support::{pallet_prelude::Get, BoundedVec};
use sp_std::vec;

//cargo build --profile=production --features runtime-benchmarks
///
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
 *  */
///
// cargo build --profile=production

#[benchmarks]
mod benches {
    use super::*;

    #[benchmark]
    fn create_claim(b: Linera<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), claim.clone());
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller, frame_system::Pallet::<T>::block_number()))
        );
        Ok(())
    }

    #[benchmark]
    fn remove_claim(b: Linera<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone())?;
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller.clone(), frame_system::Pallet::<T>::block_number()))
        );
        #[extrinsic_call]
        remove_claim(RawOrigin::Signed(caller.clone()), claim.clone());
        assert_eq!(Proofs::<T>::get(&claim), None);

        Ok(())
    }

    #[benchmark]
    fn transfer_claim(b: Linera<1, { T::MaxClaimLength::get() }>) -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let claim = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        let to: T::AccountId = account("recipient", 0, 0);

        Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone())?;
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((caller.clone(), frame_system::Pallet::<T>::block_number()))
        );
        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), to.clone());
        assert_eq!(
            Proofs::<T>::get(&claim),
            Some((to.clone(), frame_system::Pallet::<T>::block_number()))
        );

        Ok(())
    }
}
