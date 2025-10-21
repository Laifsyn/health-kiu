use std::borrow::Cow;

/// Errors that can occur in the application layer.
pub struct AppError {
    source: ErrorKind,
    context: Option<CowStr>,
}

type CowStr = Cow<'static, str>;
impl AppError {
    /// Builds a [`ServiceError`] with no context.
    pub fn new(source: impl Into<ErrorKind>) -> Self {
        Self { source: source.into(), context: None }
    }

    /// Builds a new [`ServiceError`] with additional context.
    pub fn new_with_context<S: Into<CowStr>>(
        source: impl Into<ErrorKind>,
        ctx: S,
    ) -> Self {
        Self { context: Some(ctx.into()), source: source.into() }
    }

    pub fn context<S: Into<CowStr>>(mut self, ctx: S) -> Self {
        self.context = Some(ctx.into());
        self
    }

    pub fn invalid_password() -> Self { Self::new(ErrorKind::InvalidPassword) }

    /// Returns a closure that creates a [`ServiceError`] with the given
    /// context.
    pub fn err_with<E>(ctx: impl Into<CowStr>) -> impl FnOnce(E) -> Self
    where
        E: Into<ErrorKind>,
    {
        move |e| Self::new_with_context(e.into(), ctx.into())
    }
}

impl<T> crate::ResultExt for Result<T, AppError> {
    type Context = CowStr;
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

impl<T> From<T> for AppError
where
    T: Into<ErrorKind>,
{
    fn from(value: T) -> Self { Self::new(value) }
}

#[derive(Debug, thiserror::Error)]
/// Error Kinds for [`ServiceError`]
pub enum ErrorKind {
    #[error("Not Found")]
    NotFound,
    #[error("Database Error")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Validation Error")]
    ValidationError,
    #[error("Invalid Password")]
    InvalidPassword,
    #[error("Unknown Error")]
    Unknown(#[from] color_eyre::Report),
}
