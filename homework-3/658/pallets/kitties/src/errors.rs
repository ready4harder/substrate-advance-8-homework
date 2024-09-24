use frame_support::pallet_macros::pallet_section;

/// A [`pallet_section`] that defines the errors for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod errors {
    #[pallet::error]
    pub enum Error<T> {
        InvalidKittyId,// 无效的kitty id
        NotOwner,// 不是拥有者
        SameKittyId,// 同一个kitty id
        NextKittyIdOverflow,// 下一个kitty id溢出
        KittyAlreadyOnSale,// kitty已经在售
        NotEnoughBalanceForStaking,// 余额不足
        TransferToSelf,// 无法转移给自己
        UntilBlockTooSmall,// 售卖时间太短
        KittyNotOnSale,// kitty不在售
        BidPriceTooLow,// 竞价过低
        NotEnoughBalanceForBidAndStaking,
        KittyBidLessThanOrMinimumBidAmount,
        LessBidIncrement,
        TooManyBidOnOneBlock,
        BidForSelf
    }
}
