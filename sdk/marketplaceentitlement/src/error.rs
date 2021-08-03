// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetEntitlementsError {
    pub kind: GetEntitlementsErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetEntitlementsErrorKind {
    InternalServiceErrorException(crate::error::InternalServiceErrorException),
    InvalidParameterException(crate::error::InvalidParameterException),
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetEntitlementsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetEntitlementsErrorKind::InternalServiceErrorException(_inner) => _inner.fmt(f),
            GetEntitlementsErrorKind::InvalidParameterException(_inner) => _inner.fmt(f),
            GetEntitlementsErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            GetEntitlementsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for GetEntitlementsError {
    fn code(&self) -> Option<&str> {
        GetEntitlementsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetEntitlementsError {
    pub fn new(kind: GetEntitlementsErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetEntitlementsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetEntitlementsErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display as implemented
    // by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_internal_service_error_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetEntitlementsErrorKind::InternalServiceErrorException(_)
        )
    }
    pub fn is_invalid_parameter_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetEntitlementsErrorKind::InvalidParameterException(_)
        )
    }
    pub fn is_throttling_exception(&self) -> bool {
        matches!(&self.kind, GetEntitlementsErrorKind::ThrottlingException(_))
    }
}
impl std::error::Error for GetEntitlementsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetEntitlementsErrorKind::InternalServiceErrorException(_inner) => Some(_inner),
            GetEntitlementsErrorKind::InvalidParameterException(_inner) => Some(_inner),
            GetEntitlementsErrorKind::ThrottlingException(_inner) => Some(_inner),
            GetEntitlementsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The calls to the GetEntitlements API are throttled.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ThrottlingException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ThrottlingException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ThrottlingException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThrottlingException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ThrottlingException {}
/// See [`ThrottlingException`](crate::error::ThrottlingException)
pub mod throttling_exception {
    /// A builder for [`ThrottlingException`](crate::error::ThrottlingException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException)
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message,
            }
        }
    }
}
impl ThrottlingException {
    /// Creates a new builder-style object to manufacture [`ThrottlingException`](crate::error::ThrottlingException)
    pub fn builder() -> crate::error::throttling_exception::Builder {
        crate::error::throttling_exception::Builder::default()
    }
}

/// <p>One or more parameters in your request was invalid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidParameterException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidParameterException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidParameterException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidParameterException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidParameterException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidParameterException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidParameterException {}
/// See [`InvalidParameterException`](crate::error::InvalidParameterException)
pub mod invalid_parameter_exception {
    /// A builder for [`InvalidParameterException`](crate::error::InvalidParameterException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidParameterException`](crate::error::InvalidParameterException)
        pub fn build(self) -> crate::error::InvalidParameterException {
            crate::error::InvalidParameterException {
                message: self.message,
            }
        }
    }
}
impl InvalidParameterException {
    /// Creates a new builder-style object to manufacture [`InvalidParameterException`](crate::error::InvalidParameterException)
    pub fn builder() -> crate::error::invalid_parameter_exception::Builder {
        crate::error::invalid_parameter_exception::Builder::default()
    }
}

/// <p>An internal error has occurred. Retry your request. If the problem persists, post a
/// message with details on the AWS forums.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServiceErrorException {
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServiceErrorException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServiceErrorException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServiceErrorException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServiceErrorException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServiceErrorException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServiceErrorException {}
/// See [`InternalServiceErrorException`](crate::error::InternalServiceErrorException)
pub mod internal_service_error_exception {
    /// A builder for [`InternalServiceErrorException`](crate::error::InternalServiceErrorException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalServiceErrorException`](crate::error::InternalServiceErrorException)
        pub fn build(self) -> crate::error::InternalServiceErrorException {
            crate::error::InternalServiceErrorException {
                message: self.message,
            }
        }
    }
}
impl InternalServiceErrorException {
    /// Creates a new builder-style object to manufacture [`InternalServiceErrorException`](crate::error::InternalServiceErrorException)
    pub fn builder() -> crate::error::internal_service_error_exception::Builder {
        crate::error::internal_service_error_exception::Builder::default()
    }
}