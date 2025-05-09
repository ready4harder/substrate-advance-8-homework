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
    
    
        KittyBred {
            breeder: T::AccountId,
            new_kitty_id: u32,
            kitty_id_1: u32,
            kitty_id_2: u32,
            new_kitty_data: [u8; 16],
        },

        KittyTransferred {
            from: T::AccountId,
            to: T::AccountId,
            kitty_id: u32,
        },

        KittyOnSale {
            owner: T::AccountId,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
        },
        
        KittyBid {
            bidder: T::AccountId,
            kitty_id: u32,
            price: BalanceOf<T>,
        }
    }
    
}



