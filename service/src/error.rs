use crate::id::Request;
use tokio::sync::mpsc::error::SendError;

pub enum Error {
    TokioSender(SendError<Request>),
}

impl From<SendError<Request>> for Error {
    fn from(e: SendError<Request>) -> Self {
        Error::TokioSender(e)
    }
}
