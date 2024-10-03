use frame_support::pallet_macros::pallet_section;

/// Define all events used in the pallet.
#[pallet_section]
mod events {
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated {
            creator: T::AccountId,
            kitty_id: u32,
            dna: [u8; 16],
        },
        KittyTransfered{
            from: T::AccountId,
            to: T::AccountId,
            kitty_id:u32,
            price: BalanceOf<T>,
            usd_price: Option<BalanceOf<T>>
        },
        KittyOnSaled{
            owner: T::AccountId,
            kitty_id:u32,
        },
        KittyBided{
            bidder: T::AccountId,
            kitty_id:u32,
        },
        NewPrice {
            price: u32,
            maybe_who: Option<T::AccountId>,
        },
    }
}
