use std::ops::Deref;
use std::sync::Arc;
use crate::MessageType;

#[must_use]
pub struct GuardedMessageContext<T: MessageType>(Arc<Guard<T>>);

impl<T: MessageType> GuardedMessageContext<T> {
    pub fn new(value: crate::MessageContext<T>, done_callback: impl FnOnce(&crate::MessageContext<T>) + Send + Sync + 'static) -> Self {
        Self(Arc::new(Guard {
            value,
            done_callback: Some(Box::new(done_callback))
        }))
    }
}

impl<T: MessageType> Clone for GuardedMessageContext<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: MessageType> Deref for GuardedMessageContext<T> {
    type Target = crate::MessageContext<T>;

    fn deref(&self) -> &Self::Target {
        &self.0.value
    }
}

pub struct Guard<T: MessageType> {
    value: crate::MessageContext<T>,
    done_callback: Option<Box<dyn FnOnce(&crate::MessageContext<T>) + Send + Sync>>,
}

impl<T: MessageType> Drop for Guard<T> {
    fn drop(&mut self) {
        if let Some(callback) = self.done_callback.take() {
            callback(&self.value);
        }
    }
}