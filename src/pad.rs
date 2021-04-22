use crate::Width;
use core::fmt::{Display, Error, Formatter};
use fmt_iter::repeat;

/// Pad a value that does not exceed.
///
/// Values that implement this trait are to be passed
/// to `pad` field of [`PaddedValue`](crate::PaddedValue)
/// or [`PaddedColumn`](crate::PaddedColumn).
pub trait Pad<Value: Width, PadBlock: Display> {
    /// Pad a value that does not exceed.
    fn pad(
        &self,
        formatter: &mut Formatter<'_>,
        value: &Value,
        pad_block: &PadBlock,
        pad_width: usize,
    ) -> Result<(), Error>;
}

impl<Value, PadBlock, X> Pad<Value, PadBlock> for &X
where
    Value: Width,
    PadBlock: Display,
    X: Pad<Value, PadBlock> + Sized,
{
    fn pad(
        &self,
        formatter: &mut Formatter<'_>,
        value: &Value,
        pad_block: &PadBlock,
        pad_width: usize,
    ) -> Result<(), Error> {
        X::pad(*self, formatter, value, pad_block, pad_width)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlignLeft;

impl<Value: Width, PadBlock: Display> Pad<Value, PadBlock> for AlignLeft {
    fn pad(
        &self,
        formatter: &mut Formatter<'_>,
        value: &Value,
        pad_block: &PadBlock,
        pad_width: usize,
    ) -> Result<(), Error> {
        let pad = repeat(pad_block, pad_width);
        write!(formatter, "{}{}", value, pad)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlignRight;

impl<Value: Width, PadBlock: Display> Pad<Value, PadBlock> for AlignRight {
    fn pad(
        &self,
        formatter: &mut Formatter<'_>,
        value: &Value,
        pad_block: &PadBlock,
        pad_width: usize,
    ) -> Result<(), Error> {
        let pad = repeat(pad_block, pad_width);
        write!(formatter, "{}{}", pad, value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlignCenterLeft;

impl<Value: Width, PadBlock: Display> Pad<Value, PadBlock> for AlignCenterLeft {
    fn pad(
        &self,
        formatter: &mut Formatter<'_>,
        value: &Value,
        pad_block: &PadBlock,
        pad_width: usize,
    ) -> Result<(), Error> {
        let pad = repeat(pad_block, pad_width >> 1);
        let remainder = repeat(pad_block, pad_width & 1);
        write!(formatter, "{}{}{}{}", pad, value, pad, remainder)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlignCenterRight;

impl<Value: Width, PadBlock: Display> Pad<Value, PadBlock> for AlignCenterRight {
    fn pad(
        &self,
        formatter: &mut Formatter<'_>,
        value: &Value,
        pad_block: &PadBlock,
        pad_width: usize,
    ) -> Result<(), Error> {
        let pad = repeat(pad_block, pad_width >> 1);
        let remainder = repeat(pad_block, pad_width & 1);
        write!(formatter, "{}{}{}{}", pad, remainder, value, pad)
    }
}
