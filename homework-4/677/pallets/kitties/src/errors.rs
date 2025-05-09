use frame_support::pallet_macros::pallet_section;

#[pallet_section]
mod errors {
    #[pallet::error]
    pub enum Error<T> {
        InvalidKittyId,
        NotOwner,
        SameParentId,
        KittyNotExist,
        KittyAlreadyOnSale,
        TooManyBidOnOneBlock,
        BidForSelf,
        KittyNotOnSale,
        KittyBidLessThanTheSumOfLastPriceAndMinimumBidIncrement,
        KittyBidLessThanOrMinimumBidAmount,
        NotEnoughBalanceForBidAndStaking,
        NextKittyIdOverflow,
        NotEnoughBalanceForStaking,
        TransferToSelf,
        BlockSpanTooSmall,
    }
}