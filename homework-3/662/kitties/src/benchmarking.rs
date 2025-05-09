//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as PalletKitties;
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_system::pallet_prelude::BlockNumberFor;
use frame_support::traits::Currency;
use sp_std::vec;




#[benchmarks]
mod benchmarks {
    use super::*;
    

    #[benchmark]
    fn create() -> Result<(), BenchmarkError> where u64: PartialEq<<<T as pallet::Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance> {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        create(RawOrigin::Signed(caller.clone()));

        // Verify that the kitty was created
        let kitty_id = NextKittyId::<T>::get() - 1;
        assert!(Kitties::<T>::contains_key(kitty_id));
        assert_eq!(KittyOwner::<T>::get(kitty_id), Some(caller));

        Ok(())
    }

    #[benchmark]
    fn breed() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
    
        // Create two kitties to breed
        let kitty_id_1 = NextKittyId::<T>::get();
        let kitty_id_2 = kitty_id_1 + 1;
    
        // Initialize two kitties and set the caller as their owner
        Kitties::<T>::insert(kitty_id_1, Kitty([0u8; 16]));
        Kitties::<T>::insert(kitty_id_2, Kitty([1u8; 16]));
        KittyOwner::<T>::insert(kitty_id_1, caller.clone());
        KittyOwner::<T>::insert(kitty_id_2, caller.clone());
        NextKittyId::<T>::put(kitty_id_2 + 1);
    
    
        // Now call the breed function
        #[extrinsic_call]
        breed(RawOrigin::Signed(caller.clone()), kitty_id_1, kitty_id_2);
    
        // Verify that the new kitty was created
        let new_kitty_id = NextKittyId::<T>::get() - 1;
        assert!(Kitties::<T>::contains_key(new_kitty_id));
        assert_eq!(KittyOwner::<T>::get(new_kitty_id), Some(caller));
    
        // Additional check to verify the breed operation was successful
        assert!(Kitties::<T>::contains_key(new_kitty_id)); // Ensure new kitty exists
    
        Ok(())
    }
    

    #[benchmark]
    fn transfer() -> Result<(), BenchmarkError> {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);

        // Create a kitty and assign it to the caller
        let kitty_id = NextKittyId::<T>::get();
        Kitties::<T>::insert(kitty_id, Kitty([0u8; 16]));
        KittyOwner::<T>::insert(kitty_id, caller.clone());
        NextKittyId::<T>::put(kitty_id + 1);

        #[extrinsic_call]
        transfer(RawOrigin::Signed(caller.clone()), recipient.clone(), kitty_id);

        // Verify that the kitty was transferred
        assert_eq!(KittyOwner::<T>::get(kitty_id), Some(recipient));

        Ok(())
    }

    #[benchmark]
    fn sale() -> Result<(), BenchmarkError>  {
        let caller: T::AccountId = whitelisted_caller();
        let until_block: BlockNumberFor<T> = frame_system::Pallet::<T>::block_number() + 100u32.into();

        // Create a kitty and assign it to the caller
        let kitty_id = NextKittyId::<T>::get();
        Kitties::<T>::insert(kitty_id, Kitty([0u8; 16]));
        KittyOwner::<T>::insert(kitty_id, caller.clone());
        NextKittyId::<T>::put(kitty_id + 1);

        #[extrinsic_call]
        sale(RawOrigin::Signed(caller.clone()), kitty_id, until_block);

        // Verify that the kitty is on sale
        assert!(KittiesOnSale::<T>::contains_key(kitty_id));
        assert_eq!(KittiesOnSale::<T>::get(kitty_id), Some((caller, until_block)));

        Ok(())
    }

    #[benchmark]
    fn bid() -> Result<(), BenchmarkError> {     
        let caller: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("owner", 0, 0); // Different account for the owner
        let kitty_id = 1; // Ensure this ID exists and is on sale
        let price: BalanceOf<T> = T::Currency::minimum_balance() + 10u32.into(); // Example price
        
         // Set up a kitty for sale if it doesn't already exist
            if !Kitties::<T>::contains_key(kitty_id) {
                // Initialize the kitty
                Kitties::<T>::insert(kitty_id, Kitty([0u8; 16]));
                // Mark it for sale
                KittiesOnSale::<T>::insert(kitty_id, (owner.clone(), frame_system::Pallet::<T>::block_number() + 100u32.into()));
                KittyOwner::<T>::insert(kitty_id, owner.clone()); // Ensure owner is different from caller
            }
        
        // Ensure the kitty is marked for sale
        assert!(KittiesOnSale::<T>::contains_key(kitty_id));
        
        // Reserve the required amount for the bid
        T::Currency::make_free_balance_be(&caller, price * 10u32.into());
        
        // Ensure the caller's balance is sufficient
        assert!(T::Currency::free_balance(&caller) >= price);
        
        // Check if the owner is correctly set
        assert_eq!(KittyOwner::<T>::get(kitty_id), Some(owner.clone()));
        
        #[extrinsic_call]
        bid(RawOrigin::Signed(caller.clone()), kitty_id, price);
        
        // Verify that the bid was registered
        assert!(KittiesBid::<T>::contains_key(kitty_id));
        let bids = KittiesBid::<T>::get(kitty_id).unwrap();
        assert!(bids.iter().any(|(bidder, bid_price)| bidder == &caller && bid_price == &price));
        
        Ok(())
    }

    
    impl_benchmark_test_suite!(
        PalletKitties,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
