use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::{MessageContext, MessageType};
use crate::scheduler::Scheduler;
use crate::node_interface::NodeInterface;
use crate::network_adapter::NetworkAdapter;

pub struct Network<T: MessageType> {
    pub(crate) nodes: Arc<Mutex<HashMap<usize, Box<dyn NodeInterface<T>>>>>,
    pub(crate) next_node_id: Arc<AtomicUsize>,
    pub(crate) scheduler: Arc<Scheduler>,
}

impl<T: MessageType> Default for Network<T> {
    fn default() -> Self {
        Self {
            next_node_id: Arc::new(AtomicUsize::new(0)),
            nodes: Arc::new(Mutex::new(HashMap::new())),
            scheduler: Arc::new(Scheduler::new()),
        }
    }
}

impl<T: MessageType> Network<T> {
    pub fn new() -> Self {
        Self {
            next_node_id: Arc::new(AtomicUsize::new(0)),
            nodes: Arc::new(Mutex::new(HashMap::new())),
            scheduler: Arc::new(Scheduler::new()),
        }
    }

    pub fn add_node(&mut self, mut node: impl NodeInterface<T>) -> usize {
        let node_id = self.next_node_id.fetch_add(1, Ordering::Relaxed);
        node.start(NetworkAdapter::new(self.clone(), node_id));
        self.nodes.lock().expect("Failed to lock nodes").insert(node_id, Box::new(node));
        node_id
    }

    pub fn transport_message(&self, from: usize, to: usize, message: T, delay: u64) {
        self.scheduler.schedule(delay, {
            let network = self.clone();
            let nodes = self.nodes.clone();

            move || {
                let mut nodes = nodes.lock().expect("failed to lock nodes");
                nodes.get_mut(&to).unwrap().receive_message(message, MessageContext::new(network, from, to));
            }
        });
    }

    pub fn broadcast(&mut self, from: usize, message: T, delay: u64) {
        for to in self.nodes.lock().expect("lock failed").keys().copied() {
            self.transport_message(from, to, message.clone(), delay);
        }
    }

    pub fn run(&mut self) {
        for (_, node) in self.nodes.lock().expect("Failed to lock nodes").iter_mut() {
            node.run();
        }

        self.scheduler.run();
    }
}

impl<T: MessageType> Clone for Network<T> {
    fn clone(&self) -> Self {
        Self {
            next_node_id: self.next_node_id.clone(),
            nodes: self.nodes.clone(),
            scheduler: self.scheduler.clone(),
        }
    }
}