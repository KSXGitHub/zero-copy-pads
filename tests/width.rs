use padded_column::Width;
use pretty_assertions::assert_eq;

#[test]
fn width_of_str_indirect_references() {
    assert_eq!(Width::width("abcdef"), 6);
    assert_eq!(Width::width(&"abcdef"), 6);
    assert_eq!(Width::width(&&"abcdef"), 6);
    assert_eq!(Width::width(&&&"abcdef"), 6);
    assert_eq!(Width::width(&&&&"abcdef"), 6);
}

#[test]
#[cfg(feature = "std")]
fn width_owned_string_indirect_references() {
    let owned_string = "abcdef".to_string();
    assert_eq!(Width::width(&owned_string), 6);
    assert_eq!(Width::width(&&owned_string), 6);
    assert_eq!(Width::width(&&&owned_string), 6);
    assert_eq!(Width::width(&&&&owned_string), 6);
}
