use crate::{
    AlignCenterLeft, AlignCenterRight, AlignLeft, AlignRight, IgnoreExcess, PaddedValue, Width,
};

#[cfg(feature = "std")]
use crate::{PaddedColumn, PaddedColumnIter};

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
        ) ->  PaddedColumnIter<ValueList::Item, char, $alignment>
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
    /// Pad space characters to the right of a value.
    ///
    /// **When `value.width()` is not greater than `total_width`,
    /// add space characters to the right of `value` to make its
    /// width equals to `total_width`:**
    ///
    /// ```
    /// use zero_copy_pads::align_left;
    /// # use pretty_assertions::assert_eq;
    /// let value = "abc";
    /// let padded_value = align_left(value, 5);
    /// assert_eq!(padded_value.to_string(), "abc  ");
    /// ```
    ///
    /// **When `value.width()` is greater than `total_width`,
    /// display `value` as is:**
    ///
    /// ```
    /// use zero_copy_pads::align_left;
    /// let value = "abcdefghi";
    /// let padded_value = align_left(value, 5);
    /// assert_eq!(padded_value.to_string(), value);
    /// ```
    align_left = AlignLeft
}

single_fn! {
    /// Pad space characters to the left of a value.
    ///
    /// **When `value.width()` is not greater than `total_width`,
    /// add space characters to the left of `value` to make its
    /// width equals to `total_width`:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_right;
    /// let value = "abc";
    /// let padded_value = align_right(value, 5);
    /// assert_eq!(padded_value.to_string(), "  abc");
    /// ```
    ///
    /// **When `value.width()` is greater than `total_width`,
    /// display `value` as is:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_right;
    /// let value = "abcdefghi";
    /// let padded_value = align_right(value, 5);
    /// assert_eq!(padded_value.to_string(), value);
    /// ```
    align_right = AlignRight
}

single_fn! {
    /// Pad space characters both side of a value with the remainder
    /// block (if any) in the right.
    ///
    /// **When `value.width()` is not greater than `total_width`
    /// and `total_width - value.width()` is an even number,
    /// center the value in a space of `total_width`:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_center_left;
    /// let value = "abc";
    /// let padded_value = align_center_left(value, 7);
    /// assert_eq!(padded_value.to_string(), "  abc  ");
    /// ```
    ///
    /// **When `value.width()` is not greater than `total_width`
    /// and `total_width - value.width()` is an odd number
    /// center the value in a space of `total_width` but with
    /// 1 remainder block to the right:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_center_left;
    /// let value = "abc";
    /// let padded_value = align_center_left(value, 8);
    /// assert_eq!(padded_value.to_string(), "  abc   ");
    /// ```
    ///
    /// **When `value.width()` is greater than `total_width`,
    /// display `value` as is:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_center_left;
    /// let value = "abcdefghi";
    /// let padded_value = align_center_left(value, 5);
    /// assert_eq!(padded_value.to_string(), value);
    /// ```
    align_center_left = AlignCenterLeft
}

single_fn! {
    /// Pad space characters both side of a value with the remainder
    /// block (if any) in the left.
    ///
    /// **When `value.width()` is not greater than `total_width`
    /// and `total_width - value.width()` is an even number,
    /// center the value in a space of `total_width`:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_center_right;
    /// let value = "abc";
    /// let padded_value = align_center_right(value, 7);
    /// assert_eq!(padded_value.to_string(), "  abc  ");
    /// ```
    ///
    /// **When `value.width()` is not greater than `total_width`
    /// and `total_width - value.width()` is an odd number
    /// center the value in a space of `total_width` but with
    /// 1 remainder block to the left:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_center_right;
    /// let value = "abc";
    /// let padded_value = align_center_right(value, 8);
    /// assert_eq!(padded_value.to_string(), "   abc  ");
    /// ```
    ///
    /// **When `value.width()` is greater than `total_width`,
    /// display `value` as is:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_center_right;
    /// let value = "abcdefghi";
    /// let padded_value = align_center_right(value, 5);
    /// assert_eq!(padded_value.to_string(), value);
    /// ```
    align_center_right = AlignCenterRight
}

multi_fn! {
    /// Pad space characters to the right of every value so that they all share the same width.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_column_left;
    /// use pipe_trait::Pipe;
    /// let values = vec![
    ///     "Rust", "C", "C++", "C#", "JavaScript",
    ///     "TypeScript", "Java", "Kotlin", "Go",
    /// ];
    /// let padded_values: Vec<_> = values
    ///     .iter()
    ///     .pipe(align_column_left)
    ///     .into_iter()
    ///     .map(|x| x.to_string())
    ///     .collect();
    /// let expected = [
    ///     "Rust      ", "C         ", "C++       ",
    ///     "C#        ", "JavaScript", "TypeScript",
    ///     "Java      ", "Kotlin    ", "Go        ",
    /// ];
    /// assert_eq!(padded_values, expected);
    /// ```
    align_column_left = AlignLeft
}

multi_fn! {
    /// Pad space characters to the left of every value so that they all share the same width.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_column_right;
    /// use pipe_trait::Pipe;
    /// let values = vec![
    ///     "Rust", "C", "C++", "C#", "JavaScript",
    ///     "TypeScript", "Java", "Kotlin", "Go",
    /// ];
    /// let padded_values: Vec<_> = values
    ///     .iter()
    ///     .pipe(align_column_right)
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
    align_column_right = AlignRight
}

multi_fn! {
    /// Pad space characters to both sides of every value so that they all share the same width.
    /// The remainder blocks will be placed at the right.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_column_center_left;
    /// use pipe_trait::Pipe;
    /// let values = vec![
    ///     "Rust", "C", "C++", "C#", "JavaScript",
    ///     "TypeScript", "Java", "Kotlin", "Go",
    /// ];
    /// let padded_values: Vec<_> = values
    ///     .iter()
    ///     .pipe(align_column_center_left)
    ///     .into_iter()
    ///     .map(|x| x.to_string())
    ///     .collect();
    /// let expected = [
    ///     "   Rust   ", "    C     ", "   C++    ",
    ///     "    C#    ", "JavaScript", "TypeScript",
    ///     "   Java   ", "  Kotlin  ", "    Go    ",
    /// ];
    /// assert_eq!(padded_values, expected);
    /// ```
    align_column_center_left = AlignCenterLeft
}

multi_fn! {
    /// Pad space characters to both sides of every value so that they all share the same width.
    /// The remainder blocks will be placed at the left.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::align_column_center_right;
    /// use pipe_trait::Pipe;
    /// let values = vec![
    ///     "Rust", "C", "C++", "C#", "JavaScript",
    ///     "TypeScript", "Java", "Kotlin", "Go",
    /// ];
    /// let padded_values: Vec<_> = values
    ///     .iter()
    ///     .pipe(align_column_center_right)
    ///     .into_iter()
    ///     .map(|x| x.to_string())
    ///     .collect();
    /// let expected = [
    ///     "   Rust   ", "     C    ", "    C++   ",
    ///     "    C#    ", "JavaScript", "TypeScript",
    ///     "   Java   ", "  Kotlin  ", "    Go    ",
    /// ];
    /// assert_eq!(padded_values, expected);
    /// ```
    align_column_center_right = AlignCenterRight
}
