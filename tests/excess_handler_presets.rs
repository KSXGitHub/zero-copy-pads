use padded_column::{ForbidExcess, IgnoreExcess, PadDirection, PaddedItem};
use pretty_assertions::assert_eq;

macro_rules! create {
    ($handle_excess:expr, $value:expr, $total_width:expr) => {
        PaddedItem {
            handle_excess: $handle_excess,
            value: $value,
            total_width: $total_width,
            pad_block: '-',
            pad_direction: PadDirection::Left,
        }
        .to_string()
    };
}

#[test]
fn forbid_excess_without_excess() {
    assert_eq!(create!(ForbidExcess, "abcdef", 9), "---abcdef");
}

#[test]
#[should_panic(expected = "value's width (9) is greater than total_width (6)")]
fn forbid_excess_with_excess() {
    create!(ForbidExcess, "abcdefghi", 6);
}

#[test]
fn ignore_excess_without_excess() {
    assert_eq!(create!(IgnoreExcess, "abcdef", 9), "---abcdef");
}

#[test]
fn ignore_excess_with_excess() {
    assert_eq!(create!(IgnoreExcess, "abcdefghi", 6), "abcdefghi");
}
