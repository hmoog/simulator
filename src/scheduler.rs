use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

pub struct Scheduler {
    events: Mutex<BinaryHeap<Reverse<Element>>>,
    clock: AtomicU64,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn schedule(&self, delay: u64, callback: impl FnOnce() + Send + Sync + 'static) {
        self.events.lock().expect("failed to lock events").push(Reverse(Element {
            time: self.clock.load(Ordering::Relaxed) + delay,
            action: Box::new(callback),
        }));
    }

    pub fn run(&self) {
        while let Some(event) = self.next() {
            (event.action)();
        }
    }

    fn next(&self) -> Option<Element> {
        self.events
            .lock()
            .expect("failed to lock events")
            .pop()
            .map(|Reverse(e)| {
                self.clock.store(e.time, Ordering::Relaxed);
                e
            })
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            events: Mutex::new(BinaryHeap::new()),
            clock: AtomicU64::new(0),
        }
    }
}

struct Element {
    time: u64,
    action: Box<dyn FnOnce() + Send>,
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Element {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Element {}