use crate::{Unit, Width};
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
///
/// **Example:** Truncate to make it fit
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use zero_copy_pads::{ExcessHandler, Excess, PaddedValue, AlignRight};
/// use std::fmt::{Formatter, Result};
/// struct TruncateExcessiveString;
/// impl ExcessHandler<&str> for TruncateExcessiveString {
///     fn handle_excess(&self, excess: Excess<&str>, formatter: &mut Formatter<'_>) -> Result {
///         let mut value = excess.value.to_string();
///         value.truncate(excess.total_width);
///         write!(formatter, "{}", value)
///     }
/// }
/// let padded_value = PaddedValue {
///     handle_excess: TruncateExcessiveString,
///     value: "abcdefghi",
///     total_width: 4,
///     pad_block: ' ',
///     pad: AlignRight,
/// };
/// assert_eq!(padded_value.to_string(), "abcd");
/// ```
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
///
/// **Example:** Truncate to make it fit
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use zero_copy_pads::{ExcessHandlingFunction, Excess, PaddedValue, AlignRight};
/// use std::fmt::{Formatter, Result};
/// let truncate = ExcessHandlingFunction::<&str>(|excess, formatter| {
///     let mut value = excess.value.to_string();
///     value.truncate(excess.total_width);
///     write!(formatter, "{}", value)
/// });
/// let padded_value = PaddedValue {
///     handle_excess: truncate,
///     value: "abcdefghi",
///     total_width: 4,
///     pad_block: ' ',
///     pad: AlignRight,
/// };
/// assert_eq!(padded_value.to_string(), "abcd");
/// ```
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

/// All pre-defined zero-sized [`ExcessHandler`] types in this [crate] implement this trait.
pub trait UnitExcessHandler<Value, PadBlock = char>: Unit + ExcessHandler<Value, PadBlock>
where
    Value: Width,
    PadBlock: Display,
{
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
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct $struct_name;

        impl Unit for $struct_name {
            const VALUE: Self = $struct_name;
        }

        impl<Value: Width, PadBlock: Display> UnitExcessHandler<Value, PadBlock> for $struct_name {}

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

    /// Ignore excess, write `value` to `formatter` without padding.
    ///
    /// **When `value.width()` is not greater than `total_width`,
    /// add pads as usual:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::{PaddedValue, AlignRight, IgnoreExcess};
    /// let padded_value = PaddedValue {
    ///     handle_excess: IgnoreExcess,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     pad: AlignRight,
    /// };
    /// assert_eq!(padded_value.to_string(), "---abcdef");
    /// ```
    ///
    /// **When `value.width()` is greater than `total_width`,
    /// display `value` as is:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::{PaddedValue, AlignRight, IgnoreExcess};
    /// let padded_value = PaddedValue {
    ///     handle_excess: IgnoreExcess,
    ///     value: "abcdefghijkl",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     pad: AlignRight,
    /// };
    /// assert_eq!(padded_value.to_string(), "abcdefghijkl");
    /// ```
    struct IgnoreExcess;

    /// Create a [`ExcessHandlingFunction`] that ignores excesses.
    ///
    /// see [`IgnoreExcess`].
    fn ignore_excess;
}

preset! {
    impl |_, _| Err(Error);

    /// Forbid all excesses, raise `fmt::Error` once encounter one.
    ///
    /// **When `value.width()` is not greater than `total_width`,
    /// add pads as usual:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::{PaddedValue, AlignRight, ErrorOnExcess};
    /// let padded_value = PaddedValue {
    ///     handle_excess: ErrorOnExcess,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     pad: AlignRight,
    /// };
    /// assert_eq!(padded_value.to_string(), "---abcdef");
    /// ```
    ///
    /// **When `value.width()` is greater than `total_width`,
    /// return an [`Err`] of [`fmt::Error`](Error):**
    /// ```
    ///
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::{PaddedValue, AlignRight, ErrorOnExcess};
    /// let padded_value = PaddedValue {
    ///     handle_excess: ErrorOnExcess,
    ///     value: "abcdefghijkl",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     pad: AlignRight,
    /// };
    /// let mut output = String::new();
    /// std::fmt::write(
    ///     &mut output,
    ///     format_args!("{}", padded_value),
    /// ).unwrap_err();
    /// ```
    struct ErrorOnExcess;

    /// Create a [`ExcessHandlingFunction`] that forbids excesses.
    ///
    /// see [`ErrorOnExcess`].
    fn error_on_excess;
}

preset! {
    impl |excess, _| panic!(
        "value's width ({}) is greater than total_width ({})",
        excess.value_width, excess.total_width,
    );

    /// Forbid all excesses, panic once encounter one.
    ///
    /// **When `value.width()` is not greater than `total_width`,
    /// add pads as usual:**
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::{PaddedValue, AlignRight, PanicOnExcess};
    /// let padded_value = PaddedValue {
    ///     handle_excess: PanicOnExcess,
    ///     value: "abcdef",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     pad: AlignRight,
    /// };
    /// assert_eq!(padded_value.to_string(), "---abcdef");
    /// ```
    ///
    /// **When `value.width()` is greater than `total_width`, panic:**
    ///
    /// ```should_panic
    /// # use pretty_assertions::assert_eq;
    /// use zero_copy_pads::{PaddedValue, AlignRight, PanicOnExcess};
    /// let padded_value = PaddedValue {
    ///     handle_excess: PanicOnExcess,
    ///     value: "abcdefghijkl",
    ///     pad_block: '-',
    ///     total_width: 9,
    ///     pad: AlignRight,
    /// };
    /// assert_eq!(padded_value.to_string(), "abcdefghijkl");
    /// ```
    struct PanicOnExcess;

    /// Create a [`ExcessHandlingFunction`] that forbids excesses.
    ///
    /// see [`PanicOnExcess`].
    fn panic_on_excess;
}
