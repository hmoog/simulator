use crate::{MessageType, Network};

/// Context of a message that is being processed by a node.
pub struct MessageContext<T: MessageType> {
    /// The network that the message is being processed in.
    network: Network<T>,

    /// The id of the node that sent the message.
    from: usize,

    /// The id of the node that received the message.
    to: usize,
}

/// Context of a message that is being processed by a node.
impl <T: MessageType> MessageContext<T> {
    /// Create a new message context.
    pub fn new(network: Network<T>, from: usize, to: usize) -> Self {
        Self { network, from, to }
    }

    /// Get the id of the node that sent the message.
    pub fn from(&self) -> usize {
        self.from
    }

    /// Get the id of the node that received the message.
    pub fn to(&self) -> usize {
        self.to
    }

    /// Send a message to the node that sent the message.
    pub fn reply(&self, message: T, delay: u64) {
        self.network.transport_message(self.to, self.from, message, delay);
    }

    pub fn schedule(&self, delay: u64, callback: impl FnOnce() + Send + Sync + 'static) {
        self.network.scheduler.schedule(delay, callback);
    }
}