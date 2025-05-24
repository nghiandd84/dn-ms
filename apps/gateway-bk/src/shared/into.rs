pub trait IntoRef<T>: Sized {
    /// Converts this type into the (usually inferred) input type.
    #[must_use]
    fn into_ref(&self) -> T;
}
