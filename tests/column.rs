#![cfg(feature = "std")]
use padded_column::{Alignment, PaddedColumn};
use pretty_assertions::assert_eq;

const VALUES: &[&str] = &[
    "Rust",
    "C",
    "C++",
    "C#",
    "JavaScript",
    "TypeScript",
    "Java",
    "Kotlin",
    "Go",
];

const EXPECTED_LEFT: &[&str] = &[
    "------Rust",
    "---------C",
    "-------C++",
    "--------C#",
    "JavaScript",
    "TypeScript",
    "------Java",
    "----Kotlin",
    "--------Go",
];

const EXPECTED_RIGHT: &[&str] = &[
    "Rust------",
    "C---------",
    "C++-------",
    "C#--------",
    "JavaScript",
    "TypeScript",
    "Java------",
    "Kotlin----",
    "Go--------",
];

macro_rules! test_case {
    (
        $name:ident
        where
            direction = $direction:ident,
            values = $values:expr,
            expectation = $expected:ident,
    ) => {
        #[test]
        fn $name() {
            let values = $values;
            let padded_column = PaddedColumn {
                values: values.into_iter(),
                pad_block: '-',
                pad_direction: Alignment::$direction,
            };
            let actual: Vec<_> = padded_column.into_iter().map(|x| x.to_string()).collect();
            assert_eq!(actual, $expected);
        }
    };
}

test_case! {
    pad_left_array_of_str_slices
    where
        direction = Right,
        values = VALUES,
        expectation = EXPECTED_LEFT,
}

test_case! {
    pad_right_array_of_str_slices
    where
        direction = Left,
        values = VALUES,
        expectation = EXPECTED_RIGHT,
}

test_case! {
    pad_left_vec_of_str_indirect_references
    where
        direction = Right,
        values = VALUES.iter().collect::<Vec<&&str>>(),
        expectation = EXPECTED_LEFT,
}

test_case! {
    pad_right_vec_of_str_indirect_references
    where
        direction = Left,
        values = VALUES.iter().collect::<Vec<&&str>>(),
        expectation = EXPECTED_RIGHT,
}

test_case! {
    pad_left_vec_of_owned_strings
    where
        direction = Right,
        values = VALUES.iter().map(ToString::to_string).collect::<Vec<String>>(),
        expectation = EXPECTED_LEFT,
}

test_case! {
    pad_right_vec_of_owned_strings
    where
        direction = Left,
        values = VALUES.iter().map(ToString::to_string).collect::<Vec<String>>(),
        expectation = EXPECTED_RIGHT,
}

test_case! {
    pad_left_vec_of_str_slices
    where
        direction = Right,
        values = VALUES.iter().copied().collect::<Vec<&str>>(),
        expectation = EXPECTED_LEFT,
}

test_case! {
    pad_right_vec_of_str_slices
    where
        direction = Left,
        values = VALUES.iter().copied().collect::<Vec<&str>>(),
        expectation = EXPECTED_RIGHT,
}
