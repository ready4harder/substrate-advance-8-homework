use frame_support::pallet_macros::pallet_section;
/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let _value = Self::random_value(&who);
            // TODO 
            let current_kittty_id = NextKittyId::<T>::get();
            Kitties::<T>:::insert(current_kittty_id,Kitty(_value));
            let next_kittty_id = current_kittty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            NextKittyId::<T>::put(next_kittty_id);
            
             // Emit an event.
             Self::deposit_event(Event::KittyCreated { creator: who, index: current_kittty_id, data: _value });

            Ok(())
        }

        pub fn breed(origin: OriginFor<T>, kitty_1: u32, kitty_2: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Ok(())
        }

        pub fn transfer(origin: OriginFor<T>, kitty_id: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Ok(())
        }

        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Ok(())
        }

        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: u64) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Ok(())
        }
    }
}
