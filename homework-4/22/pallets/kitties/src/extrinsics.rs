use frame_support::pallet_macros::pallet_section;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let value = Self::random_value(&who);
            let kitty_id = NextKittyId::<T>::get();
            Kitties::<T>::insert(
                kitty_id,
                Kitty {
                    dna: value,
                    price: 0,
                },
            );

            let next_kitty_id = kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;

            NextKittyId::<T>::put(next_kitty_id);
            KittyOwner::<T>::insert(kitty_id, who.clone());

            Self::deposit_event(Event::KittyCreated {
                creator: who,
                index: kitty_id,
                data: value,
            });
            Ok(())
        }

        pub fn breed(origin: OriginFor<T>, kitty_1: u32, kitty_2: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let kitty1 = Kitties::<T>::get(kitty_1).expect("");
            let kitty2 = Kitties::<T>::get(kitty_2).expect("");

            let value = Self::breed_kitty(&who, kitty1.dna, kitty2.dna);

            let kitty_id = NextKittyId::<T>::get();
            Kitties::<T>::insert(
                kitty_id,
                Kitty {
                    dna: value,
                    price: 0,
                },
            );

            let next_kitty_id = kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;

            NextKittyId::<T>::put(next_kitty_id);
            KittyOwner::<T>::insert(kitty_id, who.clone());

            Self::deposit_event(Event::KittyBreeded {
                creator: who,
                index: kitty_id,
                data: value,
            });
            Ok(())
        }

        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, kitty_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = KittyOwner::<T>::get(kitty_id).expect("");

            ensure!(who == owner, Error::<T>::NotOwner);

            KittyOwner::<T>::insert(kitty_id, to.clone());

            Self::deposit_event(Event::Transferred {
                from: who,
                to,
                index: kitty_id,
            });

            Ok(())
        }

        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let owner = KittyOwner::<T>::get(kitty_id).expect("");
            ensure!(who == owner, Error::<T>::NotOwner);

            KittiesOnSale::<T>::insert(kitty_id, until_block);

            Self::deposit_event(Event::OnSale {
                kitty_id,
                until_block,
            });

            Ok(())
        }

        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            T::Currency::reserve(&who, price)?;

            KittiesBid::<T>::mutate(kitty_id, |bids| match bids {
                Some(bids) => {
                    bids.push((who.clone(), price));
                }
                None => *bids = Some(vec![(who.clone(), price)]),
            });

            let avg_dot_usd: BalanceOf<T> = Self::average_price().unwrap_or(0).into();
            let price_usd = price.checked_mul(&avg_dot_usd).unwrap();
            Self::deposit_event(Event::OnBid {
                kitty_id,
                price,
                price_usd,
            });

            Ok(())
        }

        /// Submit new price to the list.
        ///
        /// This method is a public function of the module and can be called from within
        /// a transaction. It appends given `price` to current list of prices.
        /// In our example the `offchain worker` will create, sign & submit a transaction that
        /// calls this function passing the price.
        ///
        /// The transaction needs to be signed (see `ensure_signed`) check, so that the caller
        /// pays a fee to execute it.
        /// This makes sure that it's not easy (or rather cheap) to attack the chain by submitting
        /// excessive transactions, but note that it doesn't ensure the price oracle is actually
        /// working and receives (and provides) meaningful data.
        /// This example is not focused on correctness of the oracle itself, but rather its
        /// purpose is to showcase offchain worker capabilities.
        #[pallet::call_index(5)]
        #[pallet::weight({0})]
        pub fn submit_price(origin: OriginFor<T>, price: u32) -> DispatchResultWithPostInfo {
            // Retrieve sender of the transaction.
            let who = ensure_signed(origin)?;
            // Add the price to the on-chain list.
            Self::add_price(Some(who), price);
            Ok(().into())
        }

        /// Submit new price to the list via unsigned transaction.
        ///
        /// Works exactly like the `submit_price` function, but since we allow sending the
        /// transaction without a signature, and hence without paying any fees,
        /// we need a way to make sure that only some transactions are accepted.
        /// This function can be called only once every `T::UnsignedInterval` blocks.
        /// Transactions that call that function are de-duplicated on the pool level
        /// via `validate_unsigned` implementation and also are rendered invalid if
        /// the function has already been called in current "session".
        ///
        /// It's important to specify `weight` for unsigned calls as well, because even though
        /// they don't charge fees, we still don't want a single block to contain unlimited
        /// number of such transactions.
        ///
        /// This example is not focused on correctness of the oracle itself, but rather its
        /// purpose is to showcase offchain worker capabilities.
        #[pallet::call_index(6)]
        #[pallet::weight({0})]
        pub fn submit_price_unsigned(
            origin: OriginFor<T>,
            _block_number: BlockNumberFor<T>,
            price: u32,
        ) -> DispatchResultWithPostInfo {
            // This ensures that the function can only be called via unsigned transaction.
            ensure_none(origin)?;
            // Add the price to the on-chain list, but mark it as coming from an empty address.
            Self::add_price(None, price);
            // now increment the block number at which we expect next unsigned transaction.
            let current_block = <system::Pallet<T>>::block_number();
            <NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());
            Ok(().into())
        }

        #[pallet::call_index(7)]
        #[pallet::weight({0})]
        pub fn submit_price_unsigned_with_signed_payload(
            origin: OriginFor<T>,
            price_payload: PricePayload<T::Public, BlockNumberFor<T>>,
            _signature: T::Signature,
        ) -> DispatchResultWithPostInfo {
            // This ensures that the function can only be called via unsigned transaction.
            ensure_none(origin)?;
            // Add the price to the on-chain list, but mark it as coming from an empty address.
            Self::add_price(None, price_payload.price);
            // now increment the block number at which we expect next unsigned transaction.
            let current_block = <system::Pallet<T>>::block_number();
            <NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());
            Ok(().into())
        }
    }
}
