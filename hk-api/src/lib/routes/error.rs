use core::fmt;
use std::borrow::Cow;
use std::fmt::Display;

use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub struct ApiError {
    pub context: Option<Cow<'static, str>>,
    pub source: ErrorKind,
}

impl ApiError {
    /// Creates a new `ApiError` with the given `ErrorKind`.
    pub fn new(source: impl Into<ErrorKind>) -> Self {
        Self { context: None, source: source.into() }
    }

    /// Attaches additional context to the `ApiError`.
    pub fn context<S: Into<Cow<'static, str>>>(mut self, ctx: S) -> Self {
        self.context = Some(ctx.into());
        self
    }

    pub fn with_context<S: Into<Cow<'static, str>>>(
        source: ErrorKind,
        ctx: S,
    ) -> Self {
        Self { context: Some(ctx.into()), source }
    }

    /// Helper function to convert Compatible errors.
    ///
    /// ```rs, ignore
    /// #[derive(Debug)]
    /// struct FooError;
    /// impl fmt::Display for FooError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "Foo Error ocurred.")
    ///     }
    /// }
    /// impl From<FooError> for ErrorKind {
    ///     fn from(value: FooError) -> Self { unimplemented!() }
    /// }
    /// fn return_foo_error() -> Result<(), FooError> { unimplemented!() }
    /// fn main() {
    ///     let foo: Result<(), ApiError> =
    ///         return_foo_error().map_err(ApiError::from_err);
    /// }
    /// ```
    pub fn from_err<T>(err: T) -> ApiError
    where
        T: Into<ErrorKind> + Display,
    {
        let context = err.to_string().into();
        ApiError { context: Some(context), source: err.into() }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { context, source } = self;

        match context {
            Some(ctx) => write!(f, "{ctx}: {source}"),
            None => write!(f, "{source}"),
        }
    }
}

impl serde::Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let Self { context, source } = self;
        let count = if context.is_some() { 2 } else { 1 };
        let mut state = serializer.serialize_struct("ApiError", count)?;

        if let Some(ctx) = context {
            state.serialize_field("context", ctx)?;
        }
        state.serialize_field("error", &source)?;
        state.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test() {
        let err = ApiError {
            context: Some("Failed to fetch user".into()),
            source: ErrorKind::Internal,
        };
        let json = serde_json::to_string(&err).unwrap();
        println!("{}", json);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("Database error")]
    Database(#[from] sea_orm::DbErr),
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Bad request")]
    BadRequest,
    #[error("Internal server error")]
    Internal,
}
impl ErrorKind {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Database(_) => "Database",
            Self::NotFound => "NotFound",
            Self::Unauthorized => "Unauthorized",
            Self::BadRequest => "BadRequest",
            Self::Internal => "Internal",
        }
    }
}

impl Serialize for ErrorKind {
    /// Maps variant's names to `type`, and [`fmt::Display`] implementation to
    /// `message`.
    ///
    /// ```
    /// {
    ///  "type": "Variant Name",
    ///  "message": "Display implementation"
    /// }
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Database", 2)?;
        state.serialize_field("type", self.kind())?;

        match self {
            Self::Database(e) => {
                state.serialize_field("message", &e.to_string())?;
            }
            _ => {
                state.serialize_field("message", &self.to_string())?;
            }
        }
        state.end()
    }
}
