use frame_support::pallet_macros::pallet_section;
/// A [`pallet_section`] that defines the errors for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod config {
    #[pallet::config]
    pub trait Config:  CreateSignedTransaction<Call<Self>> + frame_system::Config {
        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
        /// A random value generator.
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

        type Currency:Currency<Self::AccountId>+ReservableCurrency<Self::AccountId>;

        // 创建kitty的押金
        #[pallet::constant]
        type KittyStake: Get<BalanceOf<Self>>;

        #[pallet::constant]
        // 表示在发送交易后的宽限期,用于控制交易发送的频率，避免在短时间内发送过多的交易
		type GracePeriod: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        // 允许的最大价格数量
		type MaxPrices: Get<u32>;
    }
}
