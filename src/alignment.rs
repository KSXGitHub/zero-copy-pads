/// Where the place the pad blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Pad to the right, content to the left.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{Alignment::Left, PaddedValue, ForbidExcess};
    /// let padded_value = PaddedValue {
    ///     alignment: Left,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     handle_excess: ForbidExcess,
    /// };
    /// assert_eq!(padded_value.to_string(), "abcdef---");
    /// ```
    Left,

    /// Pad to the left, content to the right.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{Alignment::Right, PaddedValue, ForbidExcess};
    /// let padded_value = PaddedValue {
    ///     alignment: Right,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     handle_excess: ForbidExcess,
    /// };
    /// assert_eq!(padded_value.to_string(), "---abcdef");
    /// ```
    Right,
}
