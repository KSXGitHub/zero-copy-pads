use crate::{AlignCenterLeft, AlignCenterRight, AlignLeft, AlignRight, Pad, Width};
use core::fmt::{Display, Error, Formatter};

/// Where the place the pad blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Pad to the right, content to the left.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{Alignment::Left, PaddedValue, PanicOnExcess};
    /// let padded_value = PaddedValue {
    ///     pad: Left,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     handle_excess: PanicOnExcess,
    /// };
    /// assert_eq!(padded_value.to_string(), "abcdef---");
    /// ```
    Left,

    /// Pad to the left, content to the right.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{Alignment::Right, PaddedValue, PanicOnExcess};
    /// let padded_value = PaddedValue {
    ///     pad: Right,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     handle_excess: PanicOnExcess,
    /// };
    /// assert_eq!(padded_value.to_string(), "---abcdef");
    /// ```
    Right,

    /// Pad to both sides, place content in the middle, but shift to the left one
    /// block if it can't be exactly central.
    ///
    /// **Example:**
    ///
    /// ```
    /// # #[cfg(feature = "std")] fn main() {
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{Alignment::CenterLeft, PaddedColumn, PanicOnExcess};
    /// let values = [
    ///     "Rust", "C", "C++", "C#", "JavaScript",
    ///     "TypeScript", "Java", "Kotlin", "Go",
    /// ];
    /// let padded_column = PaddedColumn {
    ///     pad: CenterLeft,
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
    CenterLeft,

    /// Pad to both sides, place content in the middle, but shift to the right one
    /// block if it can't be exactly central.
    ///
    /// **Example:**
    ///
    /// ```
    /// # #[cfg(feature = "std")] fn main() {
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{Alignment::CenterRight, PaddedColumn, PanicOnExcess};
    /// let values = [
    ///     "Rust", "C", "C++", "C#", "JavaScript",
    ///     "TypeScript", "Java", "Kotlin", "Go",
    /// ];
    /// let padded_column = PaddedColumn {
    ///     pad: CenterRight,
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
    CenterRight,
}

impl<Value: Width, PadBlock: Display> Pad<Value, PadBlock> for Alignment {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
        value: &Value,
        pad_block: &PadBlock,
        pad_width: usize,
    ) -> Result<(), Error> {
        use Alignment::*;
        macro_rules! call {
            ($name:ident) => {
                $name.fmt(formatter, value, pad_block, pad_width)
            };
        }
        match *self {
            Left => call!(AlignLeft),
            Right => call!(AlignRight),
            CenterLeft => call!(AlignCenterLeft),
            CenterRight => call!(AlignCenterRight),
        }
    }
}
