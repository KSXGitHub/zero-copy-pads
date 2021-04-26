#![cfg(feature = "std")]

use crate::{Alignment, PaddedValue, PanicOnExcess, Width};
use derive_builder::Builder;
use std::{cmp::max, collections::LinkedList, fmt::Display};

/// Pad all values in a collection to be of same (maximum) width.
///
/// **Required features:** `std`
///
/// **Key traits:**
/// * [`IntoIterator`]: Build an iterator of padded values.
///
/// **Example:**
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use zero_copy_pads::{PaddedColumn, AlignRight};
/// let values = vec![
///     "Rust", "C", "C++", "C#", "JavaScript",
///     "TypeScript", "Java", "Kotlin", "Go",
/// ];
/// let padded_column = PaddedColumn {
///     values: values.iter(),
///     pad_block: ' ',
///     pad: AlignRight,
/// };
/// let padded_values: Vec<_> = padded_column
///     .into_iter()
///     .map(|x| x.to_string())
///     .collect();
/// let expected = [
///     "      Rust", "         C", "       C++",
///     "        C#", "JavaScript", "TypeScript",
///     "      Java", "    Kotlin", "        Go",
/// ];
/// assert_eq!(padded_values, expected);
/// ```
#[derive(Debug, Clone, Copy, Builder)]
pub struct PaddedColumn<ValueIter, PadBlock = char, Pad = Alignment>
where
    ValueIter: Iterator,
    ValueIter::Item: Width,
    PadBlock: Display + Copy,
    Pad: crate::Pad<ValueIter::Item, PadBlock> + Copy,
{
    /// Values to be padded.
    pub values: ValueIter,
    /// Block of the pad (expected to have width of 1).
    pub pad_block: PadBlock,
    /// Where to place the pad.
    pub pad: Pad,
}

impl<ValueIter, PadBlock, Pad> IntoIterator for PaddedColumn<ValueIter, PadBlock, Pad>
where
    ValueIter: Iterator,
    ValueIter::Item: Width,
    PadBlock: Display + Copy,
    Pad: crate::Pad<ValueIter::Item, PadBlock> + Copy,
{
    type Item = PaddedValue<ValueIter::Item, PadBlock, PanicOnExcess, Pad>;
    type IntoIter = PaddedColumnIter<ValueIter::Item, PadBlock, Pad>;
    fn into_iter(self) -> Self::IntoIter {
        let PaddedColumn {
            values,
            pad_block,
            pad,
        } = self;
        let mut iter = PaddedColumnIter {
            value_list: LinkedList::new(),
            total_width: 0,
            pad_block,
            pad,
        };
        for value in values {
            iter.push_back(value);
        }
        iter
    }
}

/// Iterator created by calling [`into_iter`](IntoIterator::into_iter) on [`PaddedColumn`].
///
/// **Required features:** `std`
#[derive(Debug, Clone)]
pub struct PaddedColumnIter<Value, PadBlock = char, Pad = Alignment>
where
    Value: Width,
    PadBlock: Display + Copy,
    Pad: crate::Pad<Value, PadBlock> + Copy,
{
    value_list: LinkedList<Value>,
    pad_block: PadBlock,
    pad: Pad,
    total_width: usize,
}

impl<Value, PadBlock, Pad> PaddedColumnIter<Value, PadBlock, Pad>
where
    Value: Width,
    PadBlock: Display + Copy,
    Pad: crate::Pad<Value, PadBlock> + Copy,
{
    /// Add a value to the column.
    /// If width of the new value is greater than the current total_width,
    /// set it as the new total_width.
    pub fn push_back(&mut self, value: Value) {
        self.total_width = max(self.total_width, value.width());
        self.value_list.push_back(value);
    }

    /// Pad block that was used in the construction of [`PaddedColumn`].
    pub fn pad_block(&self) -> PadBlock {
        self.pad_block
    }

    /// Padding method that was used in the construction of [`PaddedColumn`].
    pub fn pad(&self) -> Pad {
        self.pad
    }

    /// Maximum width of all items that were passed to [`PaddedColumn`].
    pub fn total_width(&self) -> usize {
        self.total_width
    }
}

impl<Value, PadBlock, Pad> Iterator for PaddedColumnIter<Value, PadBlock, Pad>
where
    Value: Width,
    PadBlock: Display + Copy,
    Pad: crate::Pad<Value, PadBlock> + Copy,
{
    type Item = PaddedValue<Value, PadBlock, PanicOnExcess, Pad>;

    fn next(&mut self) -> Option<Self::Item> {
        let PaddedColumnIter {
            value_list,
            pad_block,
            pad,
            total_width,
        } = self;
        value_list.pop_front().map(|value| PaddedValue {
            value,
            pad_block: *pad_block,
            pad: *pad,
            total_width: *total_width,
            handle_excess: PanicOnExcess,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<Value, PadBlock, Pad> ExactSizeIterator for PaddedColumnIter<Value, PadBlock, Pad>
where
    Value: Width,
    PadBlock: Display + Copy,
    Pad: crate::Pad<Value, PadBlock> + Copy,
{
    fn len(&self) -> usize {
        self.value_list.len()
    }
}
