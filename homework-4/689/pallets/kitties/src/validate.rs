use frame_support::pallet_macros::pallet_section;

#[pallet_section]
mod validate {
    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        /// Validate unsigned call to this module.
        ///
        /// By default unsigned transactions are disallowed, but implementing the validator
        /// here we make sure that some particular calls (the ones produced by offchain worker)
        /// are being whitelisted and marked as valid.
        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            // Firstly let's check that we call the right function.
            if let Call::set_latest_quota_unsigned {
                quota,
            } = call
            {
                log::info!("+++++ validate transaction to update the latest price {}", quota);

                ValidTransaction::with_tag_prefix("PalletKitties")
                .priority(1) // Set the priority of the transaction
                .and_provides([b"price_update"])
                .longevity(3) // Set the number of blocks the transaction is valid for
                .propagate(true)
                .build()
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }
}