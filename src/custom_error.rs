use parallel_macro_core::TimeoutResultError;

#[derive(Debug)]
pub enum CustomError {
    Unauthorized(String),
    ResourceNotFound(String),
    Timeout(String),
    Unknown(String),
}

impl CustomError {
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        CustomError::Unauthorized(msg.into())
    }
    
    pub fn not_found(resource: impl Into<String>) -> Self {
        CustomError::ResourceNotFound(resource.into())
    }
    
    pub fn timeout(msg: impl Into<String>) -> Self {
        CustomError::Timeout(msg.into())
    }
    
    pub fn unknown(msg: impl Into<String>) -> Self {
        CustomError::Unknown(msg.into())
    }
}

// Display implementation for user-friendly error messages
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            CustomError::ResourceNotFound(resource) => write!(f, "Resource not found: {}", resource),
            CustomError::Timeout(msg) => write!(f, "Operation timed out: {}", msg),
            CustomError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

// Error trait implementation
impl std::error::Error for CustomError {}

// Now implement From<TimeoutResultError<CustomError>> for CustomError
impl From<TimeoutResultError<CustomError>> for CustomError {
    fn from(err: TimeoutResultError<CustomError>) -> Self {
        match err {
            // If it's already a CustomError, pass it through
            TimeoutResultError::Error(e) => e,
            
            // If it's a timeout, convert to the Timeout variant
            TimeoutResultError::TimedOut => CustomError::Timeout(
                "Operation did not complete within the allotted time".into()
            ),
        }
    }
}