use std::collections::VecDeque;

use crate::{Receiver, RecvError};

pub struct SelectiveReceiver<R, T> {
    inner: R,
    // Use None as tombstone for deletion.
    buffer: VecDeque<Option<T>>,
}

impl<R, T> SelectiveReceiver<R, T>
where
    R: Receiver<T>,
{
    pub fn new(inner: R) -> Self {
        let max_capacity = inner.max_capacity();
        Self {
            inner,
            buffer: VecDeque::with_capacity(max_capacity),
        }
    }
}

impl<R, T> SelectiveReceiver<R, T>
where
    R: Receiver<T> + Send,
    T: Send,
{
    pub async fn recv_selectively<F>(&mut self, predicate: F) -> Result<Option<T>, RecvError>
    where
        F: Fn(&T) -> bool,
    {
        for message in self.buffer.iter_mut() {
            if message.as_ref().map_or(false, &predicate) {
                return Ok(message.take());
            }
        }
        while self.buffer.len() < self.inner.max_capacity() {
            let next_message = self.inner.recv().await?;
            if predicate(&next_message) {
                return Ok(Some(next_message));
            }
            self.buffer.push_back(Some(next_message));
        }
        Ok(None)
    }
}

impl<R, T> Receiver<T> for SelectiveReceiver<R, T>
where
    R: Receiver<T> + Send,
    T: Send,
{
    async fn recv(&mut self) -> Result<T, RecvError> {
        while let Some(message) = self.buffer.pop_front() {
            if let Some(message) = message {
                return Ok(message);
            }
        }
        self.inner.recv().await
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn max_capacity(&self) -> usize {
        self.inner.max_capacity()
    }
}

#[cfg(test)]
mod tests;
