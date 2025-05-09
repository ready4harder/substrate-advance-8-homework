use frame_support::pallet_macros::pallet_section;

/// A [`pallet_section`] that defines the errors for a pallet.
/// This can later be imported into the pallet using [`import_section`].
/// - `ensure` !宏接受两个参数：一个布尔表达式和一个错误类型。当布尔表达式为 `false` 时，它将使用提供的错误类型来返回错误。
///  `ensure` !宏时，您可以指定具体的错误类型，这通常是在Pallet的 `Error` 枚举中定义的。这使得错误信息更加具体和清晰。
///
///
///   - `require` !宏只接受一个布尔表达式。如果表达式为 `false`，它将使用一个默认的错误类型 `Error::Require` 来返回错误。
/// `    require` !宏通常与一个字符串字面量一起使用，以提供一个简单的错误消息，但这不是强制性的。
///
#[pallet_section]
mod errors {
    #[pallet::error]
    pub enum Error<T> {
        InvalidKittyId,
        NotOwner, //
        SameKittyId,
        OverFlow,
        BlockNumberTooSmall,
        SaleIsEnd,   //无效的出价
        InvalidPrice,   //无效的出价
        BidEntriesFull, // 竞拍数据溢出；
    }
}
