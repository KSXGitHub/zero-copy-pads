use crate::{PadDirection, PaddedColumn, PaddedItem, Width, DEFAULT_EXCESS_HANDLER};
use std::iter::FromIterator;

macro_rules! single_fn {
    ($(#[$attributes:meta])* $name:ident = $direction:ident) => {
        $(#[$attributes])*
        pub fn $name<Value: Width>(value: Value, total_width: usize) -> PaddedItem<Value> {
            PaddedItem {
                value,
                total_width,
                pad_block: ' ',
                pad_direction: PadDirection::$direction,
                handle_excess: DEFAULT_EXCESS_HANDLER,
            }
        }
    };
}

macro_rules! multi_fn {
    ($(#[$attributes:meta])* $name:ident = $direction:ident) => {
        $(#[$attributes])*
        pub fn $name<Container, ValueList>(values: ValueList) -> Container
        where
            Container: FromIterator<PaddedItem<ValueList::Item>>,
            ValueList: IntoIterator,
            ValueList::Item: Width,
        {
            PaddedColumn {
                values,
                pad_block: ' ',
                pad_direction: PadDirection::$direction,
            }
            .build()
        }
    };
}

single_fn! {
    #[doc = "Pad space characters to the left of a value."]
    pad_left = Left
}

single_fn! {
    #[doc = "Pad space characters to the right of a value."]
    pad_right = Right
}

multi_fn! {
    #[doc = "Pad space characters to the left of every value so that they all share the same width."]
    pad_column_left = Left
}

multi_fn! {
    #[doc = "Pad space characters to the right of every value so that they all share the same width."]
    pad_column_right = Right
}
