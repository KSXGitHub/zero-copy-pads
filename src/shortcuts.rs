use crate::{AlignLeft, AlignRight, IgnoreExcess, PaddedValue, Width};

#[cfg(feature = "std")]
use crate::{PaddedColumn, PanicOnExcess};

macro_rules! single_fn {
    ($(#[$attributes:meta])* $name:ident = $alignment:ident) => {
        $(#[$attributes])*
        pub fn $name<Value: Width>(
            value: Value,
            total_width: usize
        ) -> PaddedValue<Value, char, IgnoreExcess, $alignment> {
            PaddedValue {
                value,
                total_width,
                pad_block: ' ',
                pad: $alignment,
                handle_excess: IgnoreExcess,
            }
        }
    };
}

macro_rules! multi_fn {
    ($(#[$attributes:meta])* $name:ident = $alignment:ident) => {
        $(#[$attributes])*
        #[cfg(feature = "std")]
        pub fn $name<ValueList>(
            values: ValueList
        ) -> impl Iterator<
            Item = PaddedValue<ValueList::Item, char, PanicOnExcess, $alignment>
        >
        where
            ValueList: Iterator,
            ValueList::Item: Width,
        {
            PaddedColumn {
                values,
                pad_block: ' ',
                pad: $alignment,
            }
            .into_iter()
        }
    };
}

single_fn! {
    #[doc = "Pad space characters to the right of a value."]
    #[doc = ""]
    #[doc = "**When `value.width()` is not greater than `total_width`,"]
    #[doc = "add space characters to the right of `value` to make its"]
    #[doc = "width equals to `total_width`:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::align_left;"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = r#"let value = "abc";"#]
    #[doc = "let padded_value = align_left(value, 5);"]
    #[doc = r#"assert_eq!(padded_value.to_string(), "abc  ");"#]
    #[doc = "```"]
    #[doc = ""]
    #[doc = "**When `value.width()` is greater than `total_width`,"]
    #[doc = "display `value` as is:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::align_left;"]
    #[doc = r#"let value = "abcdefghi";"#]
    #[doc = "let padded_value = align_left(value, 5);"]
    #[doc = "assert_eq!(padded_value.to_string(), value);"]
    #[doc = "```"]
    align_left = AlignLeft
}

single_fn! {
    #[doc = "Pad space characters to the left of a value."]
    #[doc = ""]
    #[doc = "**When `value.width()` is not greater than `total_width`,"]
    #[doc = "add space characters to the left of `value` to make its"]
    #[doc = "width equals to `total_width`:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::align_right;"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = r#"let value = "abc";"#]
    #[doc = "let padded_value = align_right(value, 5);"]
    #[doc = r#"assert_eq!(padded_value.to_string(), "  abc");"#]
    #[doc = "```"]
    #[doc = ""]
    #[doc = "**When `value.width()` is greater than `total_width`,"]
    #[doc = "display `value` as is:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::align_right;"]
    #[doc = r#"let value = "abcdefghi";"#]
    #[doc = "let padded_value = align_right(value, 5);"]
    #[doc = "assert_eq!(padded_value.to_string(), value);"]
    #[doc = "```"]
    align_right = AlignRight
}

multi_fn! {
    #[doc = "Pad space characters to the right of every value so that they all share the same width."]
    #[doc = ""]
    #[doc = "**Example:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::align_column_left;"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = r#"let values = vec!["#]
    #[doc = r#"    "Rust", "C", "C++", "C#", "JavaScript","#]
    #[doc = r#"    "TypeScript", "Java", "Kotlin", "Go","#]
    #[doc = r#"];"#]
    #[doc = "let padded_values: Vec<_> = align_column_left(values.iter())"]
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
    align_column_left = AlignLeft
}

multi_fn! {
    #[doc = "Pad space characters to the left of every value so that they all share the same width."]
    #[doc = ""]
    #[doc = "**Example:**"]
    #[doc = "```"]
    #[doc = "# use padded_column::align_column_right;"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = r#"let values = vec!["#]
    #[doc = r#"    "Rust", "C", "C++", "C#", "JavaScript","#]
    #[doc = r#"    "TypeScript", "Java", "Kotlin", "Go","#]
    #[doc = r#"];"#]
    #[doc = "let padded_values: Vec<_> = align_column_right(values.iter())"]
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
    align_column_right = AlignRight
}
