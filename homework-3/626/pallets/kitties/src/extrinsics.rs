use frame_support::pallet_macros::pallet_section;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight({0})]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let dna = Self::random_value(&who);
            Self::do_mint(&who, dna)?;
            Ok(())
        }
        #[pallet::call_index(1)]
        #[pallet::weight({0})]
        pub fn breed(origin: OriginFor<T>, kitty_1: u32, kitty_2: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            if let (Some(kitty1), Some(kitty2)) = (Kitties::<T>::get(kitty_1), Kitties::<T>::get(kitty_2)) {
                let dna = Self::breed_kitty(&who, kitty1.0, kitty2.0);
                Self::do_mint(&who, dna)?;
            }
            Ok(())
        }
        #[pallet::call_index(2)]
        #[pallet::weight({0})]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, kitty_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::do_transfer(who, to, kitty_id)?;
            Ok(())
        }
        #[pallet::call_index(3)]
        #[pallet::weight({0})]
        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
            init_amount: BalanceOf<T>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::do_sale(who, kitty_id, until_block, init_amount)?;
            Ok(())
        }
        #[pallet::call_index(4)]
        #[pallet::weight({0})]
        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::do_bid(who, kitty_id, price)?;
            Ok(())
        }
    }
}
