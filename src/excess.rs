use crate::Width;
use core::fmt::{Display, Error, Formatter};
use derive_more::{AsMut, AsRef, Deref, DerefMut, From};

/// Information about a situation where `total_width` is less than `value.width()`.
///
/// This information is to be passed to an excess handler.
#[derive(Debug, Clone, Copy)]
pub struct Excess<'a, Value, PadBlock = char>
where
    Value: Width,
    PadBlock: Display,
{
    /// The value the caused the excess.
    pub value: &'a Value,
    /// The block that was used for the pad.
    pub pad_block: &'a PadBlock,
    /// The width of the value that caused the excess.
    pub value_width: usize,
    /// The total width that was exceeded by the value.
    pub total_width: usize,
}

/// What to do when the width of the value exceeds total.
pub trait ExcessHandler<Value, PadBlock = char>
where
    Value: Width,
    PadBlock: Display,
{
    /// Handle excessive width of a value.
    fn handle_excess(
        &self,
        excess: Excess<Value, PadBlock>,
        formatter: &mut Formatter<'_>,
    ) -> Result<(), Error>;
}

type ExcessHandlingFunctionInner<Value, PadBlock> =
    fn(Excess<Value, PadBlock>, &mut Formatter<'_>) -> Result<(), Error>;

/// Turn a function (without closure) into a [`ExcessHandler`].
#[derive(Clone, Copy, AsMut, AsRef, Deref, DerefMut, From)]
pub struct ExcessHandlingFunction<Value, PadBlock = char>(
    pub ExcessHandlingFunctionInner<Value, PadBlock>,
)
where
    Value: Width,
    PadBlock: Display;

impl<Value, PadBlock> ExcessHandler<Value, PadBlock> for ExcessHandlingFunction<Value, PadBlock>
where
    Value: Width,
    PadBlock: Display,
{
    fn handle_excess(
        &self,
        excess: Excess<Value, PadBlock>,
        formatter: &mut Formatter<'_>,
    ) -> Result<(), Error> {
        self.0(excess, formatter)
    }
}

macro_rules! preset {
    (
        impl $implementation:expr;
        $(#[$struct_attr:meta])*
        struct $struct_name:ident;
        $(#[$fn_attr:meta])*
        fn $fn_name:ident;
    ) => {
        $(#[$struct_attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $struct_name;

        impl<Value, PadBlock> ExcessHandler<Value, PadBlock> for $struct_name
        where
            Value: Width,
            PadBlock: Display,
        {
            fn handle_excess(
                &self,
                excess: Excess<Value, PadBlock>,
                formatter: &mut Formatter<'_>,
            ) -> Result<(), Error> {
                let handle_excess: ExcessHandlingFunctionInner<Value, PadBlock> = $implementation;
                handle_excess(excess, formatter)
            }
        }

        impl<Value, PadBlock> From<$struct_name> for ExcessHandlingFunction<Value, PadBlock>
        where
            Value: Width,
            PadBlock: Display,
        {
            fn from(_: $struct_name) -> Self {
                ExcessHandlingFunction(|excess, formatter| {
                    $struct_name.handle_excess(excess, formatter)
                })
            }
        }

        $(#[$fn_attr])*
        pub fn $fn_name<Value, PadBlock>() -> ExcessHandlingFunction<Value, PadBlock>
        where
            Value: Width,
            PadBlock: Display,
        {
            ExcessHandlingFunction::from($struct_name)
        }
    };
}

preset! {
    impl |excess, formatter| write!(formatter, "{}", excess.value);

    #[doc = "Ignore excess, write `value` to `formatter` without padding."]
    #[doc = ""]
    #[doc = "**When `value.width()` is not greater than `total_width`,"]
    #[doc = "add pads as usual:**"]
    #[doc = "```"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = "use padded_column::{PaddedItem, PadDirection, IgnoreExcess};"]
    #[doc = "let padded_item = PaddedItem {"]
    #[doc = r#"    handle_excess: IgnoreExcess,"#]
    #[doc = r#"    value: "abcdef","#]
    #[doc = r#"    pad_block: '-',"#]
    #[doc = r#"    total_width: 9,"#]
    #[doc = r#"    pad_direction: PadDirection::Left,"#]
    #[doc = "};"]
    #[doc = r#"assert_eq!(padded_item.to_string(), "---abcdef");"#]
    #[doc = "```"]
    #[doc = ""]
    #[doc = "**When `value.width()` is greater than `total_width`,"]
    #[doc = "display `value` as is:**"]
    #[doc = "```"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = "use padded_column::{PaddedItem, PadDirection, IgnoreExcess};"]
    #[doc = "let padded_item = PaddedItem {"]
    #[doc = r#"    handle_excess: IgnoreExcess,"#]
    #[doc = r#"    value: "abcdefghijkl","#]
    #[doc = r#"    pad_block: '-',"#]
    #[doc = r#"    total_width: 9,"#]
    #[doc = r#"    pad_direction: PadDirection::Left,"#]
    #[doc = "};"]
    #[doc = r#"assert_eq!(padded_item.to_string(), "abcdefghijkl");"#]
    #[doc = "```"]
    struct IgnoreExcess;

    #[doc = "Create a [`ExcessHandlingFunction`] that ignores excesses."]
    #[doc = ""]
    #[doc = "see [`IgnoreExcess`]."]
    fn ignore_excess;
}

preset! {
    impl |excess, _| panic!(
        "value's width ({}) is greater than total_width ({})",
        excess.value_width, excess.total_width,
    );

    #[doc = "Forbid all excesses, panic once encounter one."]
    #[doc = ""]
    #[doc = "**When `value.width()` is not greater than `total_width`,"]
    #[doc = "add pads as usual:**"]
    #[doc = "```"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = "use padded_column::{PaddedItem, PadDirection, ForbidExcess};"]
    #[doc = "let padded_item = PaddedItem {"]
    #[doc = r#"    handle_excess: ForbidExcess,"#]
    #[doc = r#"    value: "abcdef","#]
    #[doc = r#"    pad_block: '-',"#]
    #[doc = r#"    total_width: 9,"#]
    #[doc = r#"    pad_direction: PadDirection::Left,"#]
    #[doc = "};"]
    #[doc = r#"assert_eq!(padded_item.to_string(), "---abcdef");"#]
    #[doc = "```"]
    #[doc = ""]
    #[doc = "**When `value.width()` is greater than `total_width`, panic:**"]
    #[doc = "```should_panic"]
    #[doc = "# use pretty_assertions::assert_eq;"]
    #[doc = "use padded_column::{PaddedItem, PadDirection, ForbidExcess};"]
    #[doc = "let padded_item = PaddedItem {"]
    #[doc = r#"    handle_excess: ForbidExcess,"#]
    #[doc = r#"    value: "abcdefghijkl","#]
    #[doc = r#"    pad_block: '-',"#]
    #[doc = r#"    total_width: 9,"#]
    #[doc = r#"    pad_direction: PadDirection::Left,"#]
    #[doc = "};"]
    #[doc = r#"assert_eq!(padded_item.to_string(), "abcdefghijkl");"#]
    #[doc = "```"]
    struct ForbidExcess;

    #[doc = "Create a [`ExcessHandlingFunction`] that forbids excesses."]
    #[doc = ""]
    #[doc = "see [`ForbidExcess`]."]
    fn forbid_excess;
}
