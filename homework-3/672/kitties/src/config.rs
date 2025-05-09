use frame_support::pallet_macros::pallet_section;

/// A [`pallet_section`] that defines the errors for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod config {
    use frame_system::offchain::AppCrypto;

    #[pallet::config]
    pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
        /// A random value generator.
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
        /// The type of currency on which the kitties are priced.
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        #[pallet::constant]
        type BreedFee: Get<BalanceOf<Self>>;
        // offchain worker

        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

        #[pallet::constant]
        type MaxPrices: Get<u32>;
        /// A grace period after we send transaction.
        ///
        /// To avoid sending too many transactions, we only attempt to send one
        /// every `GRACE_PERIOD` blocks. We use Local Storage to coordinate
        /// sending between distinct runs of this offchain worker.
        #[pallet::constant]
        type GracePeriod: Get<BlockNumberFor<Self>>;
    }
}
