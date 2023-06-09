use std::collections::VecDeque;
use std::sync::{Mutex};

use std::time::{Instant};

enum Limiters {} // TODO: ADD SIGNAL CONFIGS

pub struct Leaky {
    capacity: usize,
    rate: usize,
    last_leak_time: Instant,
    queue: Mutex<VecDeque<Instant>>,
}

impl Leaky {
    fn new(capacity: usize, rate: usize) -> Self {
        Leaky {
            capacity,
            rate,
            last_leak_time: Instant::now(),
            queue: Mutex::new(VecDeque::new()),
        }
    }

    fn enqueue(&self) -> bool {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() >= self.capacity {
            return false;
        }
        queue.push_back(Instant::now());
        true
    }

    fn dequeue(&self) {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front();
    }

    fn process(&self) {
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(self.last_leak_time);
        let tokens_to_leak = (elapsed.as_secs_f64() * self.rate as f64) as usize;

        let _queue = self.queue.lock().unwrap();
        for _ in 0..tokens_to_leak {
            // process
        }
    }
}
