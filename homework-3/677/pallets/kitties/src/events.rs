use frame_support::pallet_macros::pallet_section;

/// Define all events used in the pallet.
#[pallet_section]
mod events {
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated {
            creator: T::AccountId,
            index: u32,
            data: [u8; 16],
        },
        KittyBreeded {
            creator: T::AccountId,
            index: u32,
            data: [u8; 16],
        },

        Transferred {
            from: T::AccountId,
            to: T::AccountId,
            index: u32,
        },
        OnSale {
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
        },

        OnBid {
            kitty_id: u32,
            price: BalanceOf<T>,
        },
    }
}
