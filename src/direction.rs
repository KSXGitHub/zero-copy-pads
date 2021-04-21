/// Where the place the pad blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadDirection {
    /// Pad to the left, content to the right.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{PadDirection::Left, PaddedItem, ForbidExcess};
    /// let padded_item = PaddedItem {
    ///     pad_direction: Left,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     handle_excess: ForbidExcess,
    /// };
    /// assert_eq!(padded_item.to_string(), "---abcdef");
    /// ```
    Left,

    /// Pad to the right, content to the left.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{PadDirection::Right, PaddedItem, ForbidExcess};
    /// let padded_item = PaddedItem {
    ///     pad_direction: Right,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     handle_excess: ForbidExcess,
    /// };
    /// assert_eq!(padded_item.to_string(), "abcdef---");
    /// ```
    Right,
}
