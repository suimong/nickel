use lsp_server::{ErrorCode, ResponseError};
use lsp_types::{Position, Url};

/// Non-fatal errors generated by nls.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("scheme not supported: {0}")]
    SchemeNotSupported(String),

    #[error("invalid path: {0}")]
    InvalidPath(Url),

    #[error("file {0} not found")]
    FileNotFound(Url),

    #[error("position {pos:?} invalid for file {file}")]
    InvalidPosition { pos: Position, file: Url },

    #[error("Method not supported")]
    MethodNotFound,

    #[error("formatting failed for file {file}: {details}")]
    FormattingFailed { details: String, file: Url },
}

impl From<Error> for ResponseError {
    fn from(value: Error) -> Self {
        let code = match value {
            Error::FileNotFound(_) => ErrorCode::InvalidParams,
            Error::InvalidPosition { .. } => ErrorCode::InvalidParams,
            Error::SchemeNotSupported(_) => ErrorCode::InvalidParams,
            Error::InvalidPath(_) => ErrorCode::InvalidParams,
            Error::MethodNotFound => ErrorCode::MethodNotFound,
            Error::FormattingFailed { .. } => ErrorCode::InternalError,
        };
        ResponseError {
            code: code as i32,
            message: value.to_string(),
            data: None,
        }
    }
}
