#![cfg(feature = "std")]
use pretty_assertions::assert_eq;
use zero_copy_pads::align_column_left;

#[test]
fn test() {
    let mut actual = Vec::new();
    let mut iter = align_column_left(["", "a", "bc", "def"].iter());
    while let Some(value) = iter.next() {
        actual.push((iter.len(), value.to_string()));
    }
    let expected = [
        (3, "   ".to_string()),
        (2, "a  ".to_string()),
        (1, "bc ".to_string()),
        (0, "def".to_string()),
    ];
    assert_eq!(actual, expected);
}
