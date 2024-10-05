#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::{BoundedVec, pallet_prelude::Get};


use sp_std::vec;


/// ./solochain-template-node benchmark pallet --chain dev --steps=20 --repeat=10 --pallet=pallet_poe --extrinsic="*" --execution=wasm --wasm-execution=compiled --output=pallets/poe/src/weights.rs --template=./.maintain/frame-weight-template.hbs
/// ./solochain-template-node benchmark pallet --chain dev --execution wasm --wasm-execution-compiled --pallet pallet_poe --extrinstic "*" --steps 20 --repeat 10 --output pallets/poe/src/weights.rs
#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_claim(b: Linear<1, {T::MaxClaimLength::get()}>) -> Result<(), BenchmarkError> {
        let value: BoundedVec<u8, T::MaxClaimLength> = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        create_claim(RawOrigin::Signed(caller.clone()), value.clone());

        assert_eq!(Proofs::<T>::get(value),
                   Some((caller, frame_system::Pallet::<T>::block_number())));

        Ok(())
    }

    #[benchmark]
    fn revoke_claim(b: Linear<1, {T::MaxClaimLength::get()}>) -> Result<(), BenchmarkError> {
        let claim: BoundedVec<u8, T::MaxClaimLength> = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        let caller: T::AccountId = whitelisted_caller();

        Proofs::<T>::insert(claim.clone(), (caller.clone(), frame_system::Pallet::<T>::block_number()));
        #[extrinsic_call]
        revoke_claim(RawOrigin::Signed(caller.clone()), claim.clone());

        assert_eq!(Proofs::<T>::get(claim), None);

        Ok(())
    }

    #[benchmark]
    fn transfer_claim(b: Linear<1, {T::MaxClaimLength::get()}>) -> Result<(), BenchmarkError> {
        let claim: BoundedVec<u8, T::MaxClaimLength> = BoundedVec::try_from(vec![0; b as usize]).unwrap();
        let caller: T::AccountId = whitelisted_caller();
        let dest: T::AccountId = whitelisted_caller();

        Proofs::<T>::insert(claim.clone(), (caller.clone(), frame_system::Pallet::<T>::block_number()));
        #[extrinsic_call]
        transfer_claim(RawOrigin::Signed(caller.clone()), claim.clone(), dest.clone());

        assert_eq!(Proofs::<T>::get(claim), Some((dest, frame_system::Pallet::<T>::block_number())));

        Ok(())
    }
}
