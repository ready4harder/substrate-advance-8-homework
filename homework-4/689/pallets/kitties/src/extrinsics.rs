use frame_support::pallet_macros::pallet_section;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::create())]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let _value = Self::random_value(&who);

            Self::mint_kitty(_value, &who)?;

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::breed())]
        pub fn breed(origin: OriginFor<T>, kitty_1: u32, kitty_2: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(kitty_1 != kitty_2, Error::<T>::SameKittyId);
            ensure!(Kitties::<T>::contains_key(kitty_1) && Kitties::<T>::contains_key(kitty_2), Error::<T>::InvalidKittyId);
            ensure!(KittyOwner::<T>::get(kitty_1).unwrap() == who && KittyOwner::<T>::get(kitty_2).unwrap() == who, Error::<T>::NotOwner);

            let k1 = Kitties::<T>::get(kitty_1).unwrap();
            let k2 = Kitties::<T>::get(kitty_2).unwrap();

            let data = Self::breed_kitty(&who, k1.gene, k2.gene);
            Self::mint_kitty(data, &who)?;            

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, kitty_id: u32) -> DispatchResult {
            let from = ensure_signed(origin)?;

            ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);
            ensure!(KittyOwner::<T>::get(kitty_id).unwrap() == from, Error::<T>::NotOwner);

            Self::transfer_kitty(from, to, kitty_id)?;

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::sale())]
        pub fn sale(origin: OriginFor<T>, kitty_id: u32, price: BalanceOf<T>, until_block: BlockNumberFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);
            ensure!(!KittiesSaleInfo::<T>::contains_key(kitty_id), Error::<T>::AlreadyOnSale);
            ensure!(!KittiesBid::<T>::contains_key(kitty_id), Error::<T>::StateError);
            ensure!(KittyOwner::<T>::get(kitty_id).unwrap() == who, Error::<T>::NotOwner);

            let current_block_number = <frame_system::Pallet<T>>::block_number();
            ensure!(until_block > current_block_number, Error::<T>::WrongBlockNumber);

            Self::sell_kitty(kitty_id, price, until_block)?;

            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::bid())]
        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);
            ensure!(KittyOwner::<T>::get(kitty_id).unwrap() != who, Error::<T>::OwnerBidNotAllowed);
            ensure!(KittiesSaleInfo::<T>::contains_key(kitty_id), Error::<T>::StateError);

            Self::bid_for_kitty(who, kitty_id, price)?;

            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(0)]
        pub fn set_latest_quota_unsigned(origin: OriginFor<T>, quota: u64) -> DispatchResult {
            ensure_none(origin)?;            
            LatestQuota::<T>::put(quota);
            Ok(())
        }   
    }
}
