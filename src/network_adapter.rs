use crate::{MessageType, Network};

pub struct NetworkAdapter<T: MessageType> {
    pub network: Network<T>,
    pub id: usize,
}

impl <T: MessageType> NetworkAdapter<T> {
    pub fn new(network: Network<T>, id: usize) -> Self {
        Self { id, network }
    }

    pub fn send(&self, to: usize, message: T, delay: u64) {
        self.network.transport_message(self.id, to, message, delay);
    }
}

impl<T: MessageType> Clone for NetworkAdapter<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            network: self.network.clone(),
        }
    }
}