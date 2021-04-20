#![cfg(feature = "std")]

use crate::{forbid_excess, ExcessHandler, PadDirection, PaddedItem, Width};
use derive_builder::Builder;
use std::{collections::LinkedList, fmt::Display, iter::FromIterator};

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

impl<ValueList, PadBlock> PaddedColumn<ValueList, PadBlock>
where
    ValueList: IntoIterator,
    ValueList::Item: Width,
    PadBlock: Display + Copy,
{
    /// Create a collection of [`PaddedItem`].
    pub fn build<Container>(self) -> Container
    where
        Container: FromIterator<
            PaddedItem<ValueList::Item, PadBlock, ExcessHandler<ValueList::Item, PadBlock>>,
        >,
    {
        let mut value_list = LinkedList::new();
        let mut total_width = 0;

        let PaddedColumn {
            values,
            pad_block,
            pad_direction,
        } = self;

        for value in values {
            total_width = std::cmp::max(total_width, value.width());
            value_list.push_back(value);
        }

        value_list
            .into_iter()
            .map(|value| PaddedItem {
                value,
                pad_block,
                pad_direction,
                total_width,
                handle_excess: forbid_excess as ExcessHandler<ValueList::Item, PadBlock>,
            })
            .collect()
    }
}
