use frame_support::pallet_macros::pallet_section;

/// Define all events used in the pallet.
#[pallet_section]
mod events {
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated {
            creator: T::AccountId,
            index: KittyIndex,
            data: [u8; 16],
        },
        KittyTransferred {
            from: T::AccountId,
            to: T::AccountId,
            index: KittyIndex,
        },
        KittyBid {
            bidder: T::AccountId,
            index: KittyIndex,
            price: BalanceOf<T>,
        },
        KittyOnSale {
            index: KittyIndex,
            until: BlockNumberFor<T>,
        },
        /// Event generated when new price is accepted to contribute to the average.
        NewPrice {
            price: u32,
            maybe_who: Option<T::AccountId>,
        },
    }
}
