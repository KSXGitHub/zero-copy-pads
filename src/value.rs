use crate::{Alignment, Excess, ExcessHandler, ExcessHandlingFunction, Width};
use core::fmt::{Display, Error, Formatter};

#[cfg(feature = "std")]
use derive_builder::Builder;

/// Pad a single value.
///
/// **Key traits:**
/// * [`Display`]: Displays the padded version of the value.
///
/// **Example:** Pad dash characters to the left of a string
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use padded_column::{PaddedValue, AlignRight, PanicOnExcess};
/// let padded_value = PaddedValue {
///     value: "abcdef",
///     pad_block: '-',
///     total_width: 9,
///     pad: AlignRight,
///     handle_excess: PanicOnExcess,
/// };
/// assert_eq!(padded_value.to_string(), "---abcdef");
/// ```
///
/// **Example:** Use a [builder](PaddedValueBuilder) _(requires `std` feature)_
///
/// ```
/// # #[cfg(feature = "std")] fn main() {
/// # use pretty_assertions::assert_eq;
/// use padded_column::{PaddedValueBuilder, AlignRight, PanicOnExcess};
/// let padded_value = PaddedValueBuilder::default()
///     .value("abcdef")
///     .pad_block('-')
///     .total_width(9)
///     .pad(AlignRight)
///     .handle_excess(PanicOnExcess)
///     .build()
///     .unwrap();
/// assert_eq!(padded_value.to_string(), "---abcdef");
/// # }
/// # #[cfg(not(feature = "std"))] fn main() {}
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Builder))]
pub struct PaddedValue<
    Value,
    PadBlock = char,
    HandleExcess = ExcessHandlingFunction<Value, PadBlock>,
    Pad = Alignment,
> where
    Value: Width,
    PadBlock: Display,
    HandleExcess: ExcessHandler<Value, PadBlock>,
    Pad: crate::Pad<Value, PadBlock>,
{
    /// Value to be padded.
    pub value: Value,
    /// Block of the pad (expected to have width of 1).
    pub pad_block: PadBlock,
    /// Total width to fulfill.
    pub total_width: usize,
    /// How to pad.
    pub pad: Pad,
    /// How to write when the actual width of `value` exceeds `total_width`.
    pub handle_excess: HandleExcess,
}

impl<Value, PadBlock, HandleExcess, Pad> Display for PaddedValue<Value, PadBlock, HandleExcess, Pad>
where
    Value: Width,
    PadBlock: Display,
    HandleExcess: ExcessHandler<Value, PadBlock>,
    Pad: crate::Pad<Value, PadBlock>,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let PaddedValue {
            value,
            pad_block,
            total_width,
            pad,
            handle_excess,
        } = self;
        let total_width = *total_width;
        let value_width = value.width();
        let pad_width = if total_width >= value_width {
            total_width - value_width
        } else {
            return handle_excess.handle_excess(
                Excess {
                    value,
                    value_width,
                    total_width,
                    pad_block,
                },
                formatter,
            );
        };
        pad.fmt(formatter, value, pad_block, pad_width)
    }
}
