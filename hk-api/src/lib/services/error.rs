use std::borrow::Cow;

/// Errors that can occur in the services layer.
pub struct ServiceError {
    source: ErrorKind,
    context: Option<Cow<'static, str>>,
}

impl ServiceError {
    /// Builds a [`ServiceError`] with no context.
    pub fn new(source: impl Into<ErrorKind>) -> Self {
        Self { source: source.into(), context: None }
    }

    /// Builds a new [`ServiceError`] with additional context.
    pub fn new_with_context<S: Into<Cow<'static, str>>>(
        source: impl Into<ErrorKind>,
        ctx: S,
    ) -> Self {
        Self { context: Some(ctx.into()), source: source.into() }
    }

    pub fn context<S: Into<Cow<'static, str>>>(mut self, ctx: S) -> Self {
        self.context = Some(ctx.into());
        self
    }

    /// Returns a closure that creates a [`ServiceError`] with the given
    /// context.
    pub fn map_err_with<E>(
        ctx: impl Into<Cow<'static, str>>,
    ) -> impl FnOnce(E) -> Self
    where
        E: Into<ErrorKind>,
    {
        move |e| Self::new_with_context(e.into(), ctx.into())
    }
}

impl<T> crate::ResultExt for Result<T, ServiceError> {
    type Context = Cow<'static, str>;
    type Value = T;

    fn update_context<C: Into<Self::Context>>(self, ctx: C) -> Self {
        self.map_err(|e| e.context(ctx))
    }

    fn update_with_context<C, F>(self, f: F) -> Self
    where
        F: FnOnce() -> C,
        C: Into<Self::Context>,
    {
        self.map_err(|e| e.context(f()))
    }
}

#[derive(Debug, thiserror::Error)]
/// Error Kinds for [`ServiceError`]
pub enum ErrorKind {
    #[error("Not Found")]
    NotFound,
    #[error("Database Error")]
    DatabaseError,
    #[error("Validation Error")]
    ValidationError,
    #[error("Unknown Error")]
    Unknown(#[from] color_eyre::Report),
}
