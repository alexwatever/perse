use parse_display::ParseError;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use server_fn::ServerFnError;
use tracing::error;
use validator::ValidationErrors;

/// # Successful API Responses
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SuccessResponse {
    success: bool,
    data: String,
}

impl SuccessResponse {
    /// # Create a new success response
    ///
    /// ## Fields
    /// * `data` - The data to return in the response, as a serialised string
    ///
    /// ## Returns
    /// * `SuccessResponse` - A success response
    pub fn new(data: impl ToString) -> Self {
        Self {
            success: true,
            data: data.to_string(),
        }
    }
}

/// # SuccessResponse Conversions

impl TryFrom<SuccessResponse> for String {
    type Error = Error;

    /// # Convert a `SuccessResponse` into a string
    ///
    /// ## Fields
    /// * `response` - The response to convert to a string
    ///
    /// ## Returns
    /// * `Result<String, Error>` - A result of the conversion
    fn try_from(response: SuccessResponse) -> Result<Self, Self::Error> {
        serde_json::to_string(&response)
    }
}

// # Error Results

// Define the custom Perse error
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PerseError {
    error_type: ErrorTypes,
    data: String,
}

// Define the Perse error types
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ErrorTypes {
    InternalError,
    Conflict,
    Validation,
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
            ErrorTypes::Validation => "(400) ",
        };

        // Format the logged messsage
        message_template += &message.to_string();

        // Log the message
        error!("{}", message_template);

        Self {
            error_type,
            data: message_template,
        }
    }
}

// ## Error Conversions

impl From<PerseError> for ServerFnError {
    // Convert our `PerseError` into the Leptos `ServerFnError`
    fn from(err: PerseError) -> Self {
        // Determine the best error type
        let output: ServerFnError = match err.error_type {
            ErrorTypes::InternalError => ServerFnError::ServerError(err.data),
            ErrorTypes::Conflict => ServerFnError::ServerError(err.data),
            ErrorTypes::Validation => ServerFnError::ServerError(err.data),
        };

        output
    }
}

impl From<ParseError> for PerseError {
    // Convert a `ParseError` into a `PerseError`
    fn from(err: ParseError) -> Self {
        Self::new(ErrorTypes::Conflict, err.to_string())
    }
}

impl From<ValidationErrors> for PerseError {
    // Convert a `ValidationErrors` into a `PerseError`
    fn from(err: ValidationErrors) -> Self {
        Self::new(ErrorTypes::Validation, err.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for PerseError {
    // Convert a sqlx `Error` into a `PerseError`
    fn from(err: sqlx::Error) -> Self {
        Self::new(ErrorTypes::InternalError, format!("SQLx error: {}", err))
    }
}
