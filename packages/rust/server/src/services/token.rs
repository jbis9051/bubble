use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Limiters {
    id: str,
    isDynamic: bool,
    

} // TODO: ADD SIGNAL CONFIGS

pub struct Service {
    name: String,
    rate_limiter: Arc<TokenBucket>,
}

pub trait Actions {
    fn forward(&self, pings: i64);
    fn queue(&self, pings: i64);
}

pub struct TokenBucket {
    capacity: usize,
    refill_rate: usize,
    last_refill_time: Instant,
    buckets: Mutex<HashMap<String, usize>>, // todo map to limiter config
}

impl TokenBucket {
    fn new(capacity: usize, refill_rate: usize) -> TokenBucket {
        TokenBucket {
            capacity,
            refill_rate,
            last_refill_time: Instant::now(),
            buckets: Mutex::new(HashMap::new()),
        }
    }

    fn update(&self, buckets: &mut HashMap<String, usize>) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill_time);
        let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate as f64) as usize;
        if tokens_to_add > 0 {
            for (_, tokens) in buckets.iter_mut() {
                *tokens = std::cmp::min(self.capacity, *tokens + tokens_to_add);
            }
            self.last_refill_time = now;
        }
    }

    fn handle(&self, service: &str, num_tokens: usize) -> bool {
        let mut buckets = self.buckets.lock().unwrap();
        self.update(&mut buckets);
        let tokens_in_bucket = *buckets.get(service).unwrap_or(&0);
        if tokens_in_bucket >= num_tokens {
            buckets.insert(service.to_string(), tokens_in_bucket - num_tokens);
            true
        } else {
            false
        }
    }




}
