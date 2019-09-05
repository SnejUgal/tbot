use crate::types::chat;
use hyper::Chunk;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Represents possible errors that may happen during a method call.
#[derive(Debug)]
pub enum MethodCall {
    /// A network error.
    Network(hyper::Error),
    /// Bots API is likely to be down.
    OutOfService,
    /// Failed to parse the response.
    Parse {
        /// The response which failed to parse.
        response: Chunk,
        /// The error which parsing failed with.
        error: serde_json::Error,
    },
    /// An error returned in response.
    RequestError {
        /// A human-readable description of the error.
        description: String,
        /// The error code for this error.
        error_code: u16,
        /// The group moved to a supergroup with the following ID.
        migrate_to_chat_id: Option<chat::Id>,
        /// The bot exceeded flood threshold. You can make another request
        /// after the following amount of seconds.
        retry_after: Option<u64>,
    },
}

impl Display for MethodCall {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            MethodCall::Network(error) => write!(
                formatter,
                "A method call failed because of a network error: {}",
                error,
            ),
            MethodCall::OutOfService => write!(
                formatter,
                "A method call failed because Telegram is out of service.",
            ),
            MethodCall::Parse { response, error } => write!(
                formatter,
                "A method call failed because `tbot` failed to parse the \
                response.\n\n\

                The response was: {response:?}\n\
                The error was: {error}",
                response = response,
                error = error,
            ),
            MethodCall::RequestError {
                description,
                error_code,
                migrate_to_chat_id,
                retry_after,
            } => write!(
                formatter,
                "A method call failed because Telegram responded with an error \
                {error_code} `{description}`. Additional information:\n\n\

                - migrate_to_chat_id: {chat_id:?}\n\
                - retry_after: {retry_after:?}",
                error_code = error_code,
                description = description,
                chat_id = migrate_to_chat_id,
                retry_after = retry_after,
            ),
        }
    }
}

impl Error for MethodCall {}

impl MethodCall {
    /// Checks if `self` is `Network`.
    pub fn is_network(&self) -> bool {
        match self {
            MethodCall::Network(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `OutOfService`.
    pub fn is_out_of_service(&self) -> bool {
        match self {
            MethodCall::OutOfService => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Parse`.
    pub fn is_parse(&self) -> bool {
        match self {
            MethodCall::Parse { .. } => true,
            _ => false,
        }
    }

    /// Checks if `self` is `RequestError`.
    pub fn is_request_error(&self) -> bool {
        match self {
            MethodCall::RequestError { .. } => true,
            _ => false,
        }
    }
}

impl From<hyper::Error> for MethodCall {
    fn from(error: hyper::Error) -> Self {
        MethodCall::Network(error)
    }
}