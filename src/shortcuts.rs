use crate::{PadDirection, PaddedItem, Width, DEFAULT_EXCESS_HANDLER};

#[cfg(feature = "std")]
use crate::PaddedColumn;
#[cfg(feature = "std")]
use core::iter::FromIterator;

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
    #[doc = ""]
    #[doc = "**When `value.width()` is not greater than `total_width`,"]
    #[doc = "add space characters to the left of `value` to make its"]
    #[doc = "width equals to `total_width`:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::pad_left;"]
    #[doc = r#"let value = "abc";"#]
    #[doc = "let padded_value = pad_left(value, 5);"]
    #[doc = r#"assert_eq!(padded_value.to_string(), "  abc");"#]
    #[doc = "```"]
    #[doc = ""]
    #[doc = "**When `value.width()` is greater than `total_width`,"]
    #[doc = "display `value` as is:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::pad_left;"]
    #[doc = r#"let value = "abcdefghi";"#]
    #[doc = "let padded_value = pad_left(value, 5);"]
    #[doc = "assert_eq!(padded_value.to_string(), value);"]
    #[doc = "```"]
    pad_left = Left
}

single_fn! {
    #[doc = "Pad space characters to the right of a value."]
    #[doc = ""]
    #[doc = "**When `value.width()` is not greater than `total_width`,"]
    #[doc = "add space characters to the right of `value` to make its"]
    #[doc = "width equals to `total_width`:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::pad_right;"]
    #[doc = r#"let value = "abc";"#]
    #[doc = "let padded_value = pad_right(value, 5);"]
    #[doc = r#"assert_eq!(padded_value.to_string(), "abc  ");"#]
    #[doc = "```"]
    #[doc = ""]
    #[doc = "**When `value.width()` is greater than `total_width`,"]
    #[doc = "display `value` as is:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::pad_right;"]
    #[doc = r#"let value = "abcdefghi";"#]
    #[doc = "let padded_value = pad_right(value, 5);"]
    #[doc = "assert_eq!(padded_value.to_string(), value);"]
    #[doc = "```"]
    pad_right = Right
}

multi_fn! {
    #[cfg(feature = "std")]
    #[doc = "Pad space characters to the left of every value so that they all share the same width."]
    pad_column_left = Left
}

multi_fn! {
    #[cfg(feature = "std")]
    #[doc = "Pad space characters to the right of every value so that they all share the same width."]
    pad_column_right = Right
}
