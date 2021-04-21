#![cfg(feature = "std")]

use crate::{ForbidExcess, PadDirection, PaddedItem, Width};
use derive_builder::Builder;
use std::{cmp::max, collections::LinkedList, fmt::Display};

/// Pad all values in a collection to be of same (maximum) width.
///
/// **Example:**
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use padded_column::{PaddedColumn, PadDirection};
/// let values = vec![
///     "Rust", "C", "C++", "C#", "JavaScript",
///     "TypeScript", "Java", "Kotlin", "Go",
/// ];
/// let padded_column = PaddedColumn {
///     values: values.iter(),
///     pad_block: ' ',
///     pad_direction: PadDirection::Left,
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
pub struct PaddedColumn<ValueIter, PadBlock = char>
where
    ValueIter: Iterator,
    ValueIter::Item: Width,
    PadBlock: Display + Copy,
{
    /// Values to be padded.
    pub values: ValueIter,
    /// Block of the pad (expected to have width of 1).
    pub pad_block: PadBlock,
    /// Where to place the pad.
    pub pad_direction: PadDirection,
}

impl<ValueIter, PadBlock> IntoIterator for PaddedColumn<ValueIter, PadBlock>
where
    ValueIter: Iterator,
    ValueIter::Item: Width,
    PadBlock: Display + Copy,
{
    type Item = PaddedItem<ValueIter::Item, PadBlock, ForbidExcess>;
    type IntoIter = PaddedColumnIter<ValueIter::Item, PadBlock>;
    fn into_iter(self) -> Self::IntoIter {
        let PaddedColumn {
            values,
            pad_block,
            pad_direction,
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
            pad_direction,
            total_width,
        }
    }
}

/// Iterator created by calling [`into_iter`](IntoIterator::into_iter) on [`PaddedColumn`].
#[derive(Debug, Clone)]
pub struct PaddedColumnIter<Value, PadBlock = char>
where
    Value: Width,
    PadBlock: Display + Copy,
{
    value_iter: <LinkedList<Value> as IntoIterator>::IntoIter,
    pad_block: PadBlock,
    pad_direction: PadDirection,
    total_width: usize,
}

impl<Value, PadBlock> Iterator for PaddedColumnIter<Value, PadBlock>
where
    Value: Width,
    PadBlock: Display + Copy,
{
    type Item = PaddedItem<Value, PadBlock, ForbidExcess>;

    fn next(&mut self) -> Option<Self::Item> {
        let PaddedColumnIter {
            value_iter,
            pad_block,
            pad_direction,
            total_width,
        } = self;
        value_iter.next().map(|value| PaddedItem {
            value,
            pad_block: *pad_block,
            pad_direction: *pad_direction,
            total_width: *total_width,
            handle_excess: ForbidExcess,
        })
    }
}
