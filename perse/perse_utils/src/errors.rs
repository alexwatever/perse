use parse_display::ParseError;
use server_fn::ServerFnError;
use tracing::error;

// Define the custom Perse error
#[derive(Debug)]
pub struct PerseError {
    error_type: ErrorTypes,
    message: String,
}

// Define the Perse error types
#[derive(Debug)]
pub enum ErrorTypes {
    InternalError,
    Conflict,
}

impl PerseError {
    // Create a new error
    pub fn new(error_type: ErrorTypes, message: impl ToString) -> Self {
        // Build error message
        let mut message_template = String::from("[Perse Error] ");

        // Add the error type code
        message_template += match error_type {
            ErrorTypes::InternalError => "(500) ",
            ErrorTypes::Conflict => "(409) ",
        };

        // Format the logged messsage
        message_template += &message.to_string();

        // Log the message
        error!("{}", message_template);

        Self {
            error_type,
            message: message_template,
        }
    }
}

impl From<PerseError> for ServerFnError {
    // Convert our `PerseError` into the Leptos `ServerFnError`
    fn from(input: PerseError) -> Self {
        // Determine the best error type
        let output: ServerFnError = match input.error_type {
            ErrorTypes::InternalError => ServerFnError::ServerError(input.message),
            ErrorTypes::Conflict => ServerFnError::ServerError(input.message),
        };

        output
    }
}

impl From<ParseError> for PerseError {
    // Convert a `ParseError` into a `PerseError`
    fn from(input: ParseError) -> Self {
        Self::new(ErrorTypes::Conflict, input.to_string())
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::Error as SqlxError;

        // Convert a sqlx `Error` into a `PerseError`
        impl From<SqlxError> for PerseError {
            fn from(input: SqlxError) -> Self {
                Self::new(ErrorTypes::InternalError, format!("SQLx error: {}", input))
            }
        }
    }
}
