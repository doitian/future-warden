mod errors;
mod traits;

#[cfg(feature = "tokio")]
pub mod tokio_impl;

pub use errors::{RecvError, SendError};
pub use traits::{Receiver, Sender};
