pub use tokio::sync::mpsc::{Receiver, Sender, channel as message_queue};

use crate::{RecvError, SendError};

impl<T> crate::Sender<T> for Sender<T>
where
    T: Send,
{
    async fn send(&self, message: T) -> Result<(), SendError> {
        self.send(message).await.map_err(|_| SendError)
    }
}

impl<T> crate::Receiver<T> for Receiver<T>
where
    T: Send,
{
    async fn recv(&mut self) -> Result<T, RecvError> {
        self.recv().await.ok_or(RecvError)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn max_capacity(&self) -> usize {
        self.max_capacity()
    }
}

#[cfg(test)]
mod tests;
