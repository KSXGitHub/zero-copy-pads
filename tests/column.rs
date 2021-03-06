#![cfg(feature = "std")]

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

const EXPECTED_RIGHT: &[&str] = &[
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

const EXPECTED_CENTER_LEFT: &[&str] = &[
    "---Rust---",
    "----C-----",
    "---C++----",
    "----C#----",
    "JavaScript",
    "TypeScript",
    "---Java---",
    "--Kotlin--",
    "----Go----",
];

const EXPECTED_CENTER_RIGHT: &[&str] = &[
    "---Rust---",
    "-----C----",
    "----C++---",
    "----C#----",
    "JavaScript",
    "TypeScript",
    "---Java---",
    "--Kotlin--",
    "----Go----",
];

macro_rules! test_case {
    (
        $name:ident
        where
            pad = $pad:ident,
            alignment = $alignment:ident,
            values = $values:expr,
            expectation = $expected:ident,
    ) => {
        mod $name {
            use super::*;
            use fmt_iter::FmtIter;
            use pipe_trait::Pipe;
            use pretty_assertions::assert_eq;
            use zero_copy_pads::{$pad, Alignment, PaddedColumn, Width};

            #[test]
            fn pad_instance() {
                let values = $values;
                let padded_column = PaddedColumn {
                    values: values.into_iter(),
                    pad_block: '-',
                    pad: $pad,
                };
                let actual: Vec<_> = padded_column.into_iter().map(|x| x.to_string()).collect();
                assert_eq!(actual, $expected);
            }

            #[test]
            fn alignment() {
                let values = $values;
                let padded_column = PaddedColumn {
                    values: values.into_iter(),
                    pad_block: '-',
                    pad: Alignment::$alignment,
                };
                let actual: Vec<_> = padded_column.into_iter().map(|x| x.to_string()).collect();
                assert_eq!(actual, $expected);
            }

            #[test]
            fn fmt_iter_width() {
                let values = $values;
                let actual = PaddedColumn {
                    values: values.iter(),
                    pad_block: '-',
                    pad: $pad,
                }
                .into_iter()
                .pipe(FmtIter::from)
                .width();
                let expected = values
                    .iter()
                    .map(|x| x.len())
                    .max()
                    .expect("length of the longest string")
                    .pipe(|x| x * values.len());
                assert_eq!(actual, expected);
            }
        }
    };
}

test_case! {
    align_left_array_of_str_slices
    where
        pad = AlignLeft,
        alignment = Left,
        values = VALUES,
        expectation = EXPECTED_LEFT,
}

test_case! {
    align_right_array_of_str_slices
    where
        pad = AlignRight,
        alignment = Right,
        values = VALUES,
        expectation = EXPECTED_RIGHT,
}

test_case! {
    align_center_left_array_of_str_slices
    where
        pad = AlignCenterLeft,
        alignment = CenterLeft,
        values = VALUES,
        expectation = EXPECTED_CENTER_LEFT,
}

test_case! {
    align_center_right_array_of_str_slices
    where
        pad = AlignCenterRight,
        alignment = CenterRight,
        values = VALUES,
        expectation = EXPECTED_CENTER_RIGHT,
}

test_case! {
    align_left_vec_of_str_indirect_references
    where
        pad = AlignLeft,
        alignment = Left,
        values = VALUES.iter().collect::<Vec<&&str>>(),
        expectation = EXPECTED_LEFT,
}

test_case! {
    align_right_vec_of_str_indirect_references
    where
        pad = AlignRight,
        alignment = Right,
        values = VALUES.iter().collect::<Vec<&&str>>(),
        expectation = EXPECTED_RIGHT,
}

test_case! {
    align_center_left_vec_of_str_indirect_references
    where
        pad = AlignCenterLeft,
        alignment = CenterLeft,
        values = VALUES.iter().collect::<Vec<&&str>>(),
        expectation = EXPECTED_CENTER_LEFT,
}

test_case! {
    align_center_right_vec_of_str_indirect_references
    where
        pad = AlignCenterRight,
        alignment = CenterRight,
        values = VALUES.iter().collect::<Vec<&&str>>(),
        expectation = EXPECTED_CENTER_RIGHT,
}

test_case! {
    align_left_vec_of_owned_strings
    where
        pad = AlignLeft,
        alignment = Left,
        values = VALUES.iter().map(ToString::to_string).collect::<Vec<String>>(),
        expectation = EXPECTED_LEFT,
}

test_case! {
    align_right_vec_of_owned_strings
    where
        pad = AlignRight,
        alignment = Right,
        values = VALUES.iter().map(ToString::to_string).collect::<Vec<String>>(),
        expectation = EXPECTED_RIGHT,
}

test_case! {
    align_center_left_vec_of_owned_strings
    where
        pad = AlignCenterLeft,
        alignment = CenterLeft,
        values = VALUES.iter().map(ToString::to_string).collect::<Vec<String>>(),
        expectation = EXPECTED_CENTER_LEFT,
}

test_case! {
    align_center_right_vec_of_owned_strings
    where
        pad = AlignCenterRight,
        alignment = CenterRight,
        values = VALUES.iter().map(ToString::to_string).collect::<Vec<String>>(),
        expectation = EXPECTED_CENTER_RIGHT,
}

test_case! {
    align_left_vec_of_str_slices
    where
        pad = AlignLeft,
        alignment = Left,
        values = VALUES.iter().copied().collect::<Vec<&str>>(),
        expectation = EXPECTED_LEFT,
}

test_case! {
    align_right_vec_of_str_slices
    where
        pad = AlignRight,
        alignment = Right,
        values = VALUES.iter().copied().collect::<Vec<&str>>(),
        expectation = EXPECTED_RIGHT,
}

test_case! {
    align_center_left_vec_of_str_slices
    where
        pad = AlignCenterLeft,
        alignment = CenterLeft,
        values = VALUES.iter().copied().collect::<Vec<&str>>(),
        expectation = EXPECTED_CENTER_LEFT,
}

test_case! {
    align_center_right_vec_of_str_slices
    where
        pad = AlignCenterRight,
        alignment = CenterRight,
        values = VALUES.iter().copied().collect::<Vec<&str>>(),
        expectation = EXPECTED_CENTER_RIGHT,
}
