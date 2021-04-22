#![cfg(feature = "std")]

use crate::{Alignment, PaddedValue, PanicOnExcess, Width};
use derive_builder::Builder;
use std::{cmp::max, collections::LinkedList, fmt::Display};

/// Pad all values in a collection to be of same (maximum) width.
///
/// **Example:**
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use padded_column::{PaddedColumn, Alignment};
/// let values = vec![
///     "Rust", "C", "C++", "C#", "JavaScript",
///     "TypeScript", "Java", "Kotlin", "Go",
/// ];
/// let padded_column = PaddedColumn {
///     values: values.iter(),
///     pad_block: ' ',
///     pad: Alignment::Right,
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
        let mut value_list = LinkedList::new();
        let mut total_width = 0;
        for value in values {
            total_width = max(total_width, value.width());
            value_list.push_back(value);
        }
        PaddedColumnIter {
            value_iter: value_list.into_iter(),
            pad_block,
            pad,
            total_width,
        }
    }
}

/// Iterator created by calling [`into_iter`](IntoIterator::into_iter) on [`PaddedColumn`].
#[derive(Debug, Clone)]
pub struct PaddedColumnIter<Value, PadBlock = char, Pad = Alignment>
where
    Value: Width,
    PadBlock: Display + Copy,
    Pad: crate::Pad<Value, PadBlock> + Copy,
{
    value_iter: <LinkedList<Value> as IntoIterator>::IntoIter,
    pad_block: PadBlock,
    pad: Pad,
    total_width: usize,
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
            value_iter,
            pad_block,
            pad,
            total_width,
        } = self;
        value_iter.next().map(|value| PaddedValue {
            value,
            pad_block: *pad_block,
            pad: *pad,
            total_width: *total_width,
            handle_excess: PanicOnExcess,
        })
    }
}
