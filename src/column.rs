#![cfg(feature = "std")]

use crate::{forbid_excess, ExcessHandler, PadDirection, PaddedItem, Width};
use derive_builder::Builder;
use std::{cmp::max, collections::LinkedList, fmt::Display};

/// Pad all values in a collection to be of same (maximum) width.
#[derive(Debug, Clone, Copy, Builder)]
pub struct PaddedColumn<ValueList, PadBlock = char>
where
    ValueList: IntoIterator,
    ValueList::Item: Width,
    PadBlock: Display + Copy,
{
    /// Values to be padded.
    pub values: ValueList,
    /// Block of the pad (expected to have width of 1).
    pub pad_block: PadBlock,
    /// Where to place the pad.
    pub pad_direction: PadDirection,
}

impl<ValueList, PadBlock> IntoIterator for PaddedColumn<ValueList, PadBlock>
where
    ValueList: IntoIterator,
    ValueList::Item: Width,
    PadBlock: Display + Copy,
{
    type Item = PaddedItem<ValueList::Item, PadBlock, ExcessHandler<ValueList::Item, PadBlock>>;
    type IntoIter =
        PaddedColumnIter<<LinkedList<ValueList::Item> as IntoIterator>::IntoIter, PadBlock>;
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

/// Iterator of [`PaddedColumn`].
#[derive(Debug, Clone, Copy)]
pub struct PaddedColumnIter<ValueIter, PadBlock = char>
where
    ValueIter: Iterator,
    ValueIter::Item: Width,
    PadBlock: Display + Copy,
{
    value_iter: ValueIter,
    pad_block: PadBlock,
    pad_direction: PadDirection,
    total_width: usize,
}

impl<ValueIter, PadBlock> Iterator for PaddedColumnIter<ValueIter, PadBlock>
where
    ValueIter: Iterator,
    ValueIter::Item: Width,
    PadBlock: Display + Copy,
{
    type Item = PaddedItem<ValueIter::Item, PadBlock, ExcessHandler<ValueIter::Item, PadBlock>>;

    fn next(&mut self) -> Option<Self::Item> {
        let PaddedColumnIter {
            value_iter,
            pad_block,
            pad_direction,
            total_width,
        } = self;
        if let Some(value) = value_iter.next() {
            Some(PaddedItem {
                value,
                pad_block: *pad_block,
                pad_direction: *pad_direction,
                total_width: *total_width,
                handle_excess: forbid_excess,
            })
        } else {
            None
        }
    }
}
