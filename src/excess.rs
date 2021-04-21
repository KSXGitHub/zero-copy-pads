use crate::Width;
use core::fmt::{Display, Error, Formatter};
use derive_more::{AsMut, AsRef, Deref, DerefMut, From};

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

/// What to do when the width of the value exceeds total.
pub trait ExcessHandler<Value, PadBlock>
where
    Value: Width,
    PadBlock: Display,
{
    /// Handle excessive width of a value.
    fn handle_excess(
        &self,
        excess: Excess<Value, PadBlock>,
        formatter: &mut Formatter<'_>,
    ) -> Result<(), Error>;
}

type ExcessHandlingFunctionInner<Value, PadBlock> =
    fn(Excess<Value, PadBlock>, &mut Formatter<'_>) -> Result<(), Error>;

/// Turn a function (without closure) into a [`ExcessHandler`].
#[derive(Clone, Copy, AsMut, AsRef, Deref, DerefMut, From)]
pub struct ExcessHandlingFunction<Value, PadBlock>(
    pub ExcessHandlingFunctionInner<Value, PadBlock>,
)
where
    Value: Width,
    PadBlock: Display;

impl<Value, PadBlock> ExcessHandler<Value, PadBlock> for ExcessHandlingFunction<Value, PadBlock>
where
    Value: Width,
    PadBlock: Display,
{
    fn handle_excess(
        &self,
        excess: Excess<Value, PadBlock>,
        formatter: &mut Formatter<'_>,
    ) -> Result<(), Error> {
        self.0(excess, formatter)
    }
}

/// Ignore excess, write `value` to `formatter` without padding.
pub fn ignore_excess<Value, PadBlock>() -> ExcessHandlingFunction<Value, PadBlock>
where
    Value: Width,
    PadBlock: Display,
{
    ExcessHandlingFunction(|excess, formatter| write!(formatter, "{}", excess.value))
}

/// Forbid all excesses, panic once encounter one.
pub fn forbid_excess<Value, PadBlock>() -> ExcessHandlingFunction<Value, PadBlock>
where
    Value: Width,
    PadBlock: Display,
{
    ExcessHandlingFunction(|excess, _| {
        panic!(
            "value's width ({}) is greater than total_width ({})",
            excess.value_width, excess.total_width,
        )
    })
}
