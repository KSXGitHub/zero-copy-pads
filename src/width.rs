pub use unicode_width::{UnicodeWidthChar, UnicodeWidthStr, UNICODE_VERSION};

use core::fmt::{Display, Error, Formatter};
use derive_more::{AsMut, AsRef, Deref, DerefMut, From};

/// Value that has width.
pub trait Width: Display {
    /// Get width of the value.
    fn width(&self) -> usize;
}

impl Width for &str {
    fn width(&self) -> usize {
        UnicodeWidthStr::width(*self)
    }
}

#[cfg(feature = "std")]
impl Width for String {
    fn width(&self) -> usize {
        Width::width(&self.as_str())
    }
}

macro_rules! wrapper {
    (
        $(#[$attributes:meta])*
        $name:ident = $get_width:expr
    ) => {
        $(#[$attributes])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From)]
        pub struct $name<Inner: AsRef<str>>(Inner);

        impl<Inner: AsRef<str>> $name<Inner> {
            #[doc = "Extract the inner value."]
            pub fn into_inner(self) -> Inner {
                self.0
            }

            #[doc = "Get reference to inner value."]
            pub fn as_inner(&self) -> &'_ Inner {
                self.as_ref()
            }

            #[doc = "Get reference to inner `str`."]
            pub fn as_str(&self) -> &'_ str {
                self.as_ref()
            }
        }

        impl<Inner: AsRef<str>> Width for $name<Inner> {
            fn width(&self) -> usize {
                $get_width(self.as_str())
            }
        }

        impl<Inner: AsRef<str>> AsRef<str> for $name<Inner> {
            fn as_ref(&self) -> &'_ str {
                self.as_inner().as_ref()
            }
        }

        impl<Inner: AsRef<str>> Display for $name<Inner> {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
                write!(formatter, "{}", self.as_str())
            }
        }
    };
}

wrapper! {
    #[doc = "Treat [`UnicodeWidthStr::width`] as width."]
    UnicodeWidth = UnicodeWidthStr::width
}

wrapper! {
    #[doc = "Treat [`UnicodeWidthStr::width_cjk`] as width."]
    UnicodeWidthCjk = UnicodeWidthStr::width_cjk
}

wrapper! {
    #[doc = "Treat character count as width."]
    CharCount = |x: &str| x.chars().count()
}

wrapper! {
    #[doc = "Treat `str::len` as width."]
    Len = str::len
}
