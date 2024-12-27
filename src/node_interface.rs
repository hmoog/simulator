use crate::{MessageContext, MessageType};
use crate::network_adapter::NetworkAdapter;

pub trait NodeInterface<T: MessageType>: Send + Sync + 'static {
    fn start(&mut self, network: NetworkAdapter<T>);

    fn run(&mut self);

    fn receive_message(&mut self, message: T, ctx: MessageContext<T>);
}
