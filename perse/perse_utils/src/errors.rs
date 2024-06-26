use server_fn::ServerFnError;
use tracing::error;

// Define the custom Perse error
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

// Convert our `PerseError` into the Leptos `ServerFnError`
impl From<PerseError> for ServerFnError {
    fn from(input: PerseError) -> Self {
        // Determine the best error type
        let output: ServerFnError = match input.error_type {
            ErrorTypes::InternalError => ServerFnError::ServerError(input.message),
            ErrorTypes::Conflict => ServerFnError::ServerError(input.message),
        };

        output
    }
}
