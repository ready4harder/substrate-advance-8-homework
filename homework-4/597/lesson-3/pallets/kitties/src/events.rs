use frame_support::pallet_macros::pallet_section;

/// Define all events used in the pallet.
#[pallet_section]

mod events {
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated {
            creator: T::AccountId,
            index: u64,
            data: [u8; 16],
        },
        KittyTransferred {
            id: u64,
            from: T::AccountId,
            to: T::AccountId,
        },
        KittyBred {
            creator: T::AccountId,
            index: u64,
            data: [u8; 16],
        },
        KittySale {
            owner: T::AccountId,
            kitty_id: u64,
            until_block: BlockNumberFor<T>,
        },
        KittyBid {
            bidder: T::AccountId,
            kitty_id: u64,
            price: BalanceOf<T>,
        },
        KittySaleEnded {
            kitty_id: u64,
            owner: T::AccountId,
        },
        KittyPrice {
            price: u32,
        },
    }
}
