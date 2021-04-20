use crate::Width;
use core::fmt::{Display, Error, Formatter};

/// Information about a situation where `total_width` is less than `value.width()`.
///
/// This information is to be passed to an excess handler.
#[derive(Debug, Clone, Copy)]
pub struct Excess<'a, Value, PadBlock>
where
    Value: Width,
    PadBlock: Display,
{
    /// The value the caused the excess.
    pub value: &'a Value,
    /// The block that was used for the pad.
    pub pad_block: &'a PadBlock,
    /// The width of the value that caused the excess.
    pub value_width: usize,
    /// The total width that was exceeded by the value.
    pub total_width: usize,
}

/// Ignore excess, write `value` to `formatter` without padding.
pub fn ignore_excess<Value, PadBlock>(
    excess: Excess<Value, PadBlock>,
    formatter: &mut Formatter<'_>,
) -> Result<(), Error>
where
    Value: Width,
    PadBlock: Display,
{
    write!(formatter, "{}", excess.value)
}

/// Forbid all excesses, panic once encounter one.
pub fn forbid_excess<Value, PadBlock>(
    excess: Excess<Value, PadBlock>,
    _: &mut Formatter<'_>,
) -> Result<(), Error>
where
    Value: Width,
    PadBlock: Display,
{
    panic!(
        "value's width ({}) is greater than total_width ({})",
        excess.value_width, excess.total_width,
    )
}

pub use ignore_excess as DEFAULT_EXCESS_HANDLER;

/// Type of functions (not closures) that handles excess.
pub type ExcessHandler<Value, PadBlock> =
    fn(excess: Excess<Value, PadBlock>, &mut Formatter<'_>) -> Result<(), Error>;
