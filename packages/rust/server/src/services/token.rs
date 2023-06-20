use std::collections::HashMap;

use std::sync::{Mutex};

use std::time::{Instant};

pub struct LimiterConfig<'a> {
    name: &'a str,
    size: usize,
    rate: f64,
}

#[non_exhaustive]
struct Configs;

impl Configs {
    pub const MESSAGES: LimiterConfig<'_> = LimiterConfig {
        name: "messages",
        size: 60,
        rate: 60.0 / 0.017,
    };
    pub const CLIENT_CREATE: LimiterConfig<'_> = LimiterConfig {
        name: "messages",
        size: 2,
        rate: 1.0,
    };
    pub const CLIENT_UPDATE: LimiterConfig<'_> = LimiterConfig {
        name: "messages",
        size: 4,
        rate: 2.0,
    };
    pub const CLIENT_DELETE: LimiterConfig<'_> = LimiterConfig {
        name: "messages",
        size: 2,
        rate: 1.0,
    };

    pub const USER_REGISTRATION: LimiterConfig<'_> = LimiterConfig {
        name: "registration",
        size: 6,
        rate: 12.0,
    };
    pub const USER_CONFIRM_REGISTRATION: LimiterConfig<'_> = LimiterConfig {
        name: "confirm_registration",
        size: 5,
        rate: 10.0,
    };
    pub const USER_LOGIN_ATTEMPT: LimiterConfig<'_> = LimiterConfig {
        name: "login",
        size: 10,
        rate: 10.0 / 144.0,
    };
    pub const USER_FORGOT_PASSWORD: LimiterConfig<'_> = LimiterConfig {
        name: "forgot",
        size: 10,
        rate: 10.0 / 144.0,
    };
    pub const USER_CHANGE_EMAIL: LimiterConfig<'_> = LimiterConfig {
        name: "change",
        size: 6,
        rate: 6.0 / 10.0,
    };
    pub const USER_DELETE_USER: LimiterConfig<'_> = LimiterConfig {
        name: "delete_user",
        size: 6,
        rate: 6.0 / 10.0,
    };
}

pub struct Bucket {
    capacity: usize,
    refill_rate: f64,
    last_refill_time: Instant,
    current_tokens: usize,
}

impl Bucket {
    fn new(capacity: usize, refill_rate: f64) -> Bucket {
        Bucket {
            capacity,
            refill_rate,
            current_tokens: 0,
            last_refill_time: Instant::now(),
        }
    }
}

pub struct TokenBucket {
    buckets: Mutex<HashMap<String, Bucket>>,
}

impl TokenBucket {
    fn new() -> TokenBucket {
        TokenBucket {
            buckets: Mutex::new(HashMap::new()),
        }
    }

    fn add_bucket(&self, capacity: usize, refill_rate: f64, name: &str) {
        self.buckets
            .lock()
            .unwrap()
            .insert(name.to_string(), Bucket::new(capacity, refill_rate));
    }

    fn seed_buckets(&self, configs: &[LimiterConfig]) {
        for config in configs {
            self.add_bucket(config.size, config.rate, config.name);
        }
    }

    fn update(&self, to_update: &str) {
        let mut buckets = self.buckets.lock().unwrap();
        let mut bucket = buckets.get_mut(to_update).unwrap();
        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill_time);

        let tokens_to_add =
            (elapsed.as_secs_f64() * bucket.refill_rate / 1000000000.0) as usize;
        if tokens_to_add > 0 {
            bucket.current_tokens =
                std::cmp::min(bucket.capacity, bucket.current_tokens + tokens_to_add);
            bucket.last_refill_time = now;
        }
    }

    fn handle(&self, num_tokens: usize, to_update: &str) -> bool {
        let mut buckets = self.buckets.lock().unwrap();
        let mut bucket = buckets.get_mut(to_update).unwrap();
        self.update(to_update);

        let tokens_in_bucket = bucket.current_tokens;
        if tokens_in_bucket >= num_tokens {
            bucket.current_tokens -= num_tokens;
            true
        } else {
            false
        }
    }
}
