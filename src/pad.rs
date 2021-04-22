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

/// Pad to the right, content to the left.
///
/// **Example:**
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use padded_column::{AlignLeft, PaddedValue, PanicOnExcess};
/// let padded_value = PaddedValue {
///     pad: AlignLeft,
///     value: "abcdef",
///     pad_block: '-',
///     total_width: 9,
///     handle_excess: PanicOnExcess,
/// };
/// assert_eq!(padded_value.to_string(), "abcdef---");
/// ```
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

/// Pad to the left, content to the right.
///
/// **Example:**
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use padded_column::{AlignRight, PaddedValue, PanicOnExcess};
/// let padded_value = PaddedValue {
///     pad: AlignRight,
///     value: "abcdef",
///     pad_block: '-',
///     total_width: 9,
///     handle_excess: PanicOnExcess,
/// };
/// assert_eq!(padded_value.to_string(), "---abcdef");
/// ```
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

/// Pad to both sides, place content in the middle, but shift to the left one
/// block if it can't be exactly central.
///
/// **Example:**
///
/// ```
/// # #[cfg(feature = "std")] fn main() {
/// # use pretty_assertions::assert_eq;
/// use padded_column::{AlignCenterLeft, PaddedColumn, PanicOnExcess};
/// let values = [
///     "Rust", "C", "C++", "C#", "JavaScript",
///     "TypeScript", "Java", "Kotlin", "Go",
/// ];
/// let padded_column = PaddedColumn {
///     pad: AlignCenterLeft,
///     values: values.iter(),
///     pad_block: '-',
/// };
/// let padded_values: Vec<_> = padded_column
///     .into_iter()
///     .map(|x| x.to_string())
///     .collect();
/// let expected = [
///     "---Rust---", "----C-----", "---C++----",
///     "----C#----", "JavaScript", "TypeScript",
///     "---Java---", "--Kotlin--", "----Go----",
/// ];
/// assert_eq!(padded_values, expected);
/// # }
/// # #[cfg(not(feature = "std"))] fn main() {}
/// ```
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

/// Pad to both sides, place content in the middle, but shift to the right one
/// block if it can't be exactly central.
///
/// **Example:**
///
/// ```
/// # #[cfg(feature = "std")] fn main() {
/// # use pretty_assertions::assert_eq;
/// use padded_column::{AlignCenterRight, PaddedColumn, PanicOnExcess};
/// let values = [
///     "Rust", "C", "C++", "C#", "JavaScript",
///     "TypeScript", "Java", "Kotlin", "Go",
/// ];
/// let padded_column = PaddedColumn {
///     pad: AlignCenterRight,
///     values: values.iter(),
///     pad_block: '-',
/// };
/// let padded_values: Vec<_> = padded_column
///     .into_iter()
///     .map(|x| x.to_string())
///     .collect();
/// let expected = [
///     "---Rust---", "-----C----", "----C++---",
///     "----C#----", "JavaScript", "TypeScript",
///     "---Java---", "--Kotlin--", "----Go----",
/// ];
/// assert_eq!(padded_values, expected);
/// # }
/// # #[cfg(not(feature = "std"))] fn main() {}
/// ```
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
