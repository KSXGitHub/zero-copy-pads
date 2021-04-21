use crate::{ignore_excess, PadDirection, PaddedItem, Width};

#[cfg(feature = "std")]
use crate::PaddedColumn;

macro_rules! single_fn {
    ($(#[$attributes:meta])* $name:ident = $direction:ident) => {
        $(#[$attributes])*
        pub fn $name<Value: Width>(value: Value, total_width: usize) -> PaddedItem<Value> {
            PaddedItem {
                value,
                total_width,
                pad_block: ' ',
                pad_direction: PadDirection::$direction,
                handle_excess: ignore_excess,
            }
        }
    };
}

macro_rules! multi_fn {
    ($(#[$attributes:meta])* $name:ident = $direction:ident) => {
        $(#[$attributes])*
        #[cfg(feature = "std")]
        pub fn $name<ValueList>(values: ValueList) -> impl Iterator<Item = PaddedItem<ValueList::Item>>
        where
            ValueList: IntoIterator,
            ValueList::Item: Width,
        {
            PaddedColumn {
                values,
                pad_block: ' ',
                pad_direction: PadDirection::$direction,
            }
            .into_iter()
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
    #[doc = "# use pretty_assertions::assert_eq;"]
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
    #[doc = "# use pretty_assertions::assert_eq;"]
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
    #[doc = "Pad space characters to the left of every value so that they all share the same width."]
    #[doc = ""]
    #[doc = "**Example:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::pad_column_left;"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = r#"let values = vec!["#]
    #[doc = r#"    "Rust", "C", "C++", "C#", "JavaScript","#]
    #[doc = r#"    "TypeScript", "Java", "Kotlin", "Go","#]
    #[doc = r#"];"#]
    #[doc = "let padded_values: Vec<_> = pad_column_left(values)"]
    #[doc = "    .into_iter()"]
    #[doc = "    .map(|x| x.to_string())"]
    #[doc = "    .collect();"]
    #[doc = r#"let expected = ["#]
    #[doc = r#"    "      Rust", "         C", "       C++","#]
    #[doc = r#"    "        C#", "JavaScript", "TypeScript","#]
    #[doc = r#"    "      Java", "    Kotlin", "        Go","#]
    #[doc = r#"];"#]
    #[doc = "assert_eq!(padded_values, expected);"]
    #[doc = "```"]
    pad_column_left = Left
}

multi_fn! {
    #[doc = "Pad space characters to the right of every value so that they all share the same width."]
    #[doc = ""]
    #[doc = "**Example:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::pad_column_right;"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = r#"let values = vec!["#]
    #[doc = r#"    "Rust", "C", "C++", "C#", "JavaScript","#]
    #[doc = r#"    "TypeScript", "Java", "Kotlin", "Go","#]
    #[doc = r#"];"#]
    #[doc = "let padded_values: Vec<_> = pad_column_right(values)"]
    #[doc = "    .into_iter()"]
    #[doc = "    .map(|x| x.to_string())"]
    #[doc = "    .collect();"]
    #[doc = r#"let expected = ["#]
    #[doc = r#"    "Rust      ", "C         ", "C++       ","#]
    #[doc = r#"    "C#        ", "JavaScript", "TypeScript","#]
    #[doc = r#"    "Java      ", "Kotlin    ", "Go        ","#]
    #[doc = r#"];"#]
    #[doc = "assert_eq!(padded_values, expected);"]
    #[doc = "```"]
    pad_column_right = Right
}
