use crate::{RecvError, SendError};

pub trait Sender<T> {
    /// This method returns `SendError` if the mailbox has been closed.
    fn send(&self, message: T) -> impl Future<Output = Result<(), SendError>> + Send;
}

pub trait Receiver<T> {
    /// Receives the next value.
    ///
    /// This method returns `RecvError` if the mailbox has been closed and there are no remaining messages.
    fn recv(&mut self) -> impl Future<Output = Result<T, RecvError>> + Send;

    /// Returns the number of messages in the mailbox.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the mailbox's max capacity if it's bounded.
    fn max_capacity(&self) -> usize;
}
