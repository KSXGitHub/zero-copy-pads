/// Where the place the pad blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadDirection {
    /// Pad to the left, content to the right.
    Left,
    /// Pad to the right, content to the left.
    Right,
}
