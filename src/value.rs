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
/// use padded_column::{PaddedValue, Alignment, ForbidExcess};
/// let padded_value = PaddedValue {
///     value: "abcdef",
///     pad_block: '-',
///     total_width: 9,
///     alignment: Alignment::Right,
///     handle_excess: ForbidExcess,
/// };
/// assert_eq!(padded_value.to_string(), "---abcdef");
/// ```
///
/// **Example:** Use a [builder](PaddedValueBuilder) _(requires `std` feature)_
///
/// ```
/// # #[cfg(feature = "std")] fn main() {
/// # use pretty_assertions::assert_eq;
/// use padded_column::{PaddedValueBuilder, Alignment, ForbidExcess};
/// let padded_value = PaddedValueBuilder::default()
///     .value("abcdef")
///     .pad_block('-')
///     .total_width(9)
///     .alignment(Alignment::Right)
///     .handle_excess(ForbidExcess)
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
> where
    Value: Width,
    PadBlock: Display,
    HandleExcess: ExcessHandler<Value, PadBlock>,
{
    /// Value to be padded.
    pub value: Value,
    /// Block of the pad (expected to have width of 1).
    pub pad_block: PadBlock,
    /// Total width to fulfill.
    pub total_width: usize,
    /// Where to place the pad.
    pub alignment: Alignment,
    /// How to write when the actual width of `value` exceeds `total_width`.
    pub handle_excess: HandleExcess,
}

impl<Value, PadBlock, HandleExcess> Display for PaddedValue<Value, PadBlock, HandleExcess>
where
    Value: Width,
    PadBlock: Display,
    HandleExcess: ExcessHandler<Value, PadBlock>,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let PaddedValue {
            value,
            pad_block,
            total_width,
            alignment,
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
        let full_pad = || fmt_iter::repeat(pad_block, pad_width);
        let half_pad = || fmt_iter::repeat(pad_block, pad_width >> 1);
        let odd = || fmt_iter::repeat(pad_block, pad_width & 1);
        match *alignment {
            Alignment::Right => write!(formatter, "{}{}", full_pad(), value),
            Alignment::Left => write!(formatter, "{}{}", value, full_pad()),
            Alignment::CenterLeft => {
                write!(formatter, "{}{}{}{}", half_pad(), value, half_pad(), odd())
            }
            Alignment::CenterRight => {
                write!(formatter, "{}{}{}{}", odd(), half_pad(), value, half_pad())
            }
        }
    }
}
