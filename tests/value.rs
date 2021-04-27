use zero_copy_pads::{align_left, Width};

#[test]
fn width() {
    let expected_width = 7;
    let actual_width = align_left("abc", expected_width).width();
    assert_eq!(actual_width, expected_width);
}
