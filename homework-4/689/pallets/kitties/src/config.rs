use frame_support::pallet_macros::pallet_section;

/// A [`pallet_section`] that defines the errors for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod config {
    use frame_support::traits::ReservableCurrency;
    use frame_system::offchain::SendTransactionTypes;
    
    #[pallet::config]
    pub trait Config: frame_system::Config + SendTransactionTypes<Call<Self>> {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
        /// A random value generator.
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
        
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        #[pallet::constant]
        type KittyCost: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type BidMargin: Get<BalanceOf<Self>>;
    }
}
