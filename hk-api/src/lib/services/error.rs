use std::borrow::Cow;

/// Errors that can occur in the services layer.
pub struct ServiceError {
    source: ErrorKind,
    context: Option<Cow<'static, str>>,
}

impl ServiceError {
    pub fn new(source: impl Into<ErrorKind>) -> Self {
        Self { source: source.into(), context: None }
    }

    pub fn new_with_context<S: Into<Cow<'static, str>>>(
        source: impl Into<ErrorKind>,
        ctx: S,
    ) -> Self {
        Self { source: source.into(), context: Some(ctx.into()) }
    }

    pub fn context<S: Into<Cow<'static, str>>>(mut self, ctx: S) -> Self {
        self.context = Some(ctx.into());
        self
    }
}

#[derive(Debug, thiserror::Error)]
/// Error Kinds for [`ServiceError`]
enum ErrorKind {
    #[error("Not Found")]
    NotFound,
    #[error("Database Error")]
    DatabaseError,
    #[error("Validation Error")]
    ValidationError,
    #[error("Unknown Error")]
    Unknown(#[from] color_eyre::Report),
}
