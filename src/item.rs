use crate::{Excess, ExcessHandler, PadDirection, Width};
use core::fmt::{Display, Error, Formatter};

#[cfg(feature = "std")]
use derive_builder::Builder;

/// Pad a single value.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Builder))]
pub struct PaddedItem<Value, PadBlock = char, HandleExcess = ExcessHandler<Value, PadBlock>>
where
    Value: Width,
    PadBlock: Display,
    HandleExcess: Fn(Excess<Value, PadBlock>, &mut Formatter<'_>) -> Result<(), Error>,
{
    /// Value to be padded.
    pub value: Value,
    /// Block of the pad (expected to have width of 1).
    pub pad_block: PadBlock,
    /// Total width to fulfill.
    pub total_width: usize,
    /// Where to place the pad.
    pub pad_direction: PadDirection,
    /// How to write when the actual width of `value` exceeds `total_width`.
    pub handle_excess: HandleExcess,
}

impl<Value, PadBlock, HandleExcess> Display for PaddedItem<Value, PadBlock, HandleExcess>
where
    Value: Width,
    PadBlock: Display,
    HandleExcess: Fn(Excess<Value, PadBlock>, &mut Formatter<'_>) -> Result<(), Error>,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let PaddedItem {
            value,
            pad_block,
            total_width,
            pad_direction,
            handle_excess,
        } = self;
        let total_width = *total_width;
        let value_width = value.width();
        let pad_width = if total_width > value_width {
            total_width - value_width
        } else {
            return handle_excess(
                Excess {
                    value,
                    value_width,
                    total_width,
                    pad_block,
                },
                formatter,
            );
        };
        let pad = fmt_iter::repeat(pad_block, pad_width);
        match *pad_direction {
            PadDirection::Left => write!(formatter, "{}{}", pad, value),
            PadDirection::Right => write!(formatter, "{}{}", value, pad),
        }
    }
}
