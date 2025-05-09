use frame_support::pallet_macros::pallet_section;

/// Define all events used in the pallet.
#[pallet_section]
mod events {
    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated {
            creator: T::AccountId,
            index: u32,
            data: [u8; 16],
        },
        KittyTransferred {
            from: T::AccountId,
            to: T::AccountId,
            index: u32,
        },
        KittyOnSale {
            index: u32,
            price: BalanceOf<T>,
            until_block: BlockNumberFor<T>,
        },
        KittySold {
            index: u32,
            from: T::AccountId,
            to: T::AccountId,
            price: BalanceOf<T>,
            // USD cents
            usd_price: BalanceOf<T>,
        },
    }
}
