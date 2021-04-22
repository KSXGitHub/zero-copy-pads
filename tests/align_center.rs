use padded_column::{AlignCenterLeft, AlignCenterRight, PaddedValue, PanicOnExcess};
use pretty_assertions::assert_eq;

macro_rules! test_case {
    ($name:ident -> $pad:ident $value:literal $total_width:literal == $expected:literal) => {
        #[test]
        fn $name() {
            let padded_value = PaddedValue {
                pad: $pad,
                value: $value,
                total_width: $total_width,
                pad_block: '-',
                handle_excess: PanicOnExcess,
            };
            assert_eq!(padded_value.to_string(), $expected);
        }
    };
}

test_case!(left_odd_odd    -> AlignCenterLeft  "abc"  7 == "--abc--" );
test_case!(left_even_odd   -> AlignCenterLeft  "abcd" 7 == "-abcd--" );
test_case!(left_odd_even   -> AlignCenterLeft  "abc"  8 == "--abc---");
test_case!(left_even_even  -> AlignCenterLeft  "abcd" 8 == "--abcd--");

test_case!(right_odd_odd   -> AlignCenterRight "abc"  7 == "--abc--" );
test_case!(right_even_odd  -> AlignCenterRight "abcd" 7 == "--abcd-" );
test_case!(right_odd_even  -> AlignCenterRight "abc"  8 == "---abc--");
test_case!(right_even_even -> AlignCenterRight "abcd" 8 == "--abcd--");
