//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::v2::*;
use frame_support::traits::Currency;
use frame_system::RawOrigin;

//cargo build --profile=production --features runtime-benchmarks

/**
./target/production/solochain-template-node benchmark pallet \
--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet pallet_kitties \
--extrinsic "*" \
--steps 20 \
--repeat 10 \
--output pallets/kitties/src/weights.rs \
--template .maintain/frame-weight-template.hbs
 **/

// cargo build --profile=production
#[benchmarks]
mod benches {
    use super::*;
    #[benchmark]
    fn create() {
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        create(RawOrigin::Signed(caller.clone()));
    }
    #[benchmark]
    fn breed() {
        let caller: T::AccountId = whitelisted_caller();
        let kitty_id1 = 0;
        let kitty_id2 = 1;

        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());
        Pallet::<T>::create(RawOrigin::Signed(caller.clone()).into());

        #[extrinsic_call]
        breed(RawOrigin::Signed(caller.clone()), kitty_id1, kitty_id2);
    }
    #[benchmark]
    fn transfer() {
        let from: T::AccountId = whitelisted_caller();
        let to = account("recipient", 0, 0);
        let kitty_id = 0;
        Pallet::<T>::create(RawOrigin::Signed(from.clone()).into());
        #[extrinsic_call]
        Pallet::<T>::transfer(RawOrigin::Signed(from.clone()), kitty_id, to);
    }

    #[benchmark]
    fn sale() {
        let from: T::AccountId = whitelisted_caller();
        let kitty_id = 0;
        Pallet::<T>::create(RawOrigin::Signed(from.clone()).into());
        #[extrinsic_call]
        Pallet::<T>::sale(
            RawOrigin::Signed(from),
            kitty_id,
            10u32.into(),
            10u32.into(),
        );
    }

    #[benchmark]
    fn bid() {
        let from: T::AccountId = whitelisted_caller();
        let kitty_id = 0;
        Pallet::<T>::create(RawOrigin::Signed(from.clone()).into());
        Pallet::<T>::sale(
            RawOrigin::Signed(from).into(),
            kitty_id,
            5u32.into(),
            10u32.into(),
        );

        let bidder_id = 1;
        let bidder: T::AccountId = account("recipient", 0, 0);
        T::Currency::make_free_balance_be(&bidder, 1000u32.into());
        assert_eq!(T::Currency::free_balance(&bidder), 1000u32.into());
        #[extrinsic_call]
        Pallet::<T>::bid(RawOrigin::Signed(bidder), kitty_id, 11u32.into());
    }

    // use migration::v0;
    // #[benchmark]
    // fn migration_to_v1() {
    //     let caller: T::AccountId = whitelisted_caller();
    //     for i in 0..1000 {
    //         let dna = [i as u8; 16];
    //         let old_kitty = v0::OldKitty(dna);
    //         v0::Kitties::<T, v0::OldKitty>::insert(i as u64, old_kitty);
    //     }
    //     StorageVersion::new(0).put::<Pallet<T>>();

    //     // #[extrinsic_call]
    //     migration::migration_to_v1::<T>(RawOrigin::Signed(caller));
    // }
}
