//! Error handling utilities.

/// Extension trait for `Result` to add context to errors.
pub trait ResultExt {
    type Value;
    type Context;
    /// Adds context to the error if the result is an `Err`.
    fn update_context<C: Into<Self::Context>>(self, ctx: C) -> Self;
    /// Adds context to the error if the result is an `Err`, using a closure to
    /// generate the context.
    fn update_with_context<C, F>(self, f: F) -> Self
    where
        F: FnOnce() -> C,
        C: Into<Self::Context>;
}
