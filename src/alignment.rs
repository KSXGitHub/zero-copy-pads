/// Where the place the pad blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Pad to the right, content to the left.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{Alignment::Left, PaddedValue, PanicOnExcess};
    /// let padded_value = PaddedValue {
    ///     alignment: Left,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     handle_excess: PanicOnExcess,
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
    /// use padded_column::{Alignment::Right, PaddedValue, PanicOnExcess};
    /// let padded_value = PaddedValue {
    ///     alignment: Right,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     handle_excess: PanicOnExcess,
    /// };
    /// assert_eq!(padded_value.to_string(), "---abcdef");
    /// ```
    Right,

    /// Pad to both sides, place content in the middle, but shift to the left one
    /// block if it can't be exactly central.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{Alignment::CenterLeft, PaddedColumn, PanicOnExcess};
    /// let values = [
    ///     "Rust", "C", "C++", "C#", "JavaScript",
    ///     "TypeScript", "Java", "Kotlin", "Go",
    /// ];
    /// let padded_column = PaddedColumn {
    ///     alignment: CenterLeft,
    ///     values: values.iter(),
    ///     pad_block: '-',
    /// };
    /// let padded_values: Vec<_> = padded_column
    ///     .into_iter()
    ///     .map(|x| x.to_string())
    ///     .collect();
    /// let expected = [
    ///     "---Rust---", "----C-----", "---C++----",
    ///     "----C#----", "JavaScript", "TypeScript",
    ///     "---Java---", "--Kotlin--", "----Go----",
    /// ];
    /// assert_eq!(padded_values, expected);
    /// ```
    CenterLeft,

    /// Pad to both sides, place content in the middle, but shift to the right one
    /// block if it can't be exactly central.
    ///
    /// **Example:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use padded_column::{Alignment::CenterRight, PaddedColumn, PanicOnExcess};
    /// let values = [
    ///     "Rust", "C", "C++", "C#", "JavaScript",
    ///     "TypeScript", "Java", "Kotlin", "Go",
    /// ];
    /// let padded_column = PaddedColumn {
    ///     alignment: CenterRight,
    ///     values: values.iter(),
    ///     pad_block: '-',
    /// };
    /// let padded_values: Vec<_> = padded_column
    ///     .into_iter()
    ///     .map(|x| x.to_string())
    ///     .collect();
    /// let expected = [
    ///     "---Rust---", "-----C----", "----C++---",
    ///     "----C#----", "JavaScript", "TypeScript",
    ///     "---Java---", "--Kotlin--", "----Go----",
    /// ];
    /// assert_eq!(padded_values, expected);
    /// ```
    /// ```
    CenterRight,
}
