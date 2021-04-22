/// All zero-sized types in this [crate] implement this trait.
pub trait Unit: Sized + Default + Copy + Clone + PartialEq + Eq {
    /// Sole value of the type.
    const VALUE: Self;
}

impl Unit for () {
    const VALUE: Self = ();
}
