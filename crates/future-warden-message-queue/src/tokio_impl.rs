pub use tokio::sync::mpsc::{Receiver, Sender, channel as message_queue};

use crate::{RecvError, SendError};

impl<T> crate::Sender<T> for Sender<T>
where
    T: Send,
{
    fn send(&self, message: T) -> impl Future<Output = Result<(), SendError>> + Send {
        let fut = self.send(message);
        async { fut.await.map_err(|_| SendError) }
    }
}

impl<T> crate::Receiver<T> for Receiver<T>
where
    T: Send,
{
    fn recv(&mut self) -> impl Future<Output = Result<T, RecvError>> + Send {
        let fut = self.recv();
        async { fut.await.ok_or(RecvError) }
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
