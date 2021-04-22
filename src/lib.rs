//! Padding/aligning values without heap allocation.
//!
//! **Features:**
//! * `std` _(default feature)_:
//!   - Disable `#![no_std]`.
//!   - Enable features that require heap allocation.
//!
//! **Usage:**
//!
//! Almost all items are documented with example codes, such as
//! [`align_left`], [`align_right`], [`align_center_left`], [`align_center_right`],
//! [`align_column_left`], [`align_column_right`],
//! [`align_column_center_left`], [`align_column_center_right`],
//! [`PaddedValue`], [`PaddedColumn`], [`Alignment`],
//! [`AlignLeft`], [`AlignRight`], [`AlignCenterLeft`], [`AlignCenterRight`],
//! etc.

#![cfg_attr(not(feature = "std"), no_std)]

mod alignment;
mod excess;
mod pad;
mod shortcuts;
mod value;
mod width;

pub use alignment::*;
pub use excess::*;
pub use pad::*;
pub use shortcuts::*;
pub use value::*;
pub use width::*;

#[cfg(feature = "std")]
mod column;

#[cfg(feature = "std")]
pub use column::*;
