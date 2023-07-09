use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use std::thread;

use std::time::{Duration, Instant};

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

#[derive(Copy, Clone)]
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

    fn update(&self, to_update: &str) -> Result<(), &'static str> { // error handling
        let mut buckets = self.buckets.lock().unwrap();
        let mut bucket = buckets.get_mut(to_update).ok_or("Bucket not found")?;
        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill_time);

        let tokens_to_add = (elapsed.as_secs_f64() * bucket.refill_rate) as usize;
        if tokens_to_add > 0 {
            bucket.current_tokens =
                std::cmp::min(bucket.capacity, bucket.current_tokens + tokens_to_add);
            bucket.last_refill_time = now;
        }
        Ok(())
    }

    fn handle(&self, num_tokens: usize, to_update: &str) -> Result<bool, &'static str> {
        let mut buckets = self.buckets.lock().unwrap();
        let mut bucket = buckets.get_mut(to_update).ok_or("Bucket not found")?;
        self.update(to_update)?;

        let tokens_in_bucket = bucket.current_tokens;
        if tokens_in_bucket >= num_tokens {
            bucket.current_tokens -= num_tokens;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Clone)]
struct Service {
    name: String,
    token_bucket: Arc<TokenBucket>,
}

impl Service { // axum
    fn new(name: String, token_bucket: Arc<TokenBucket>) -> Service {
        Service { name, token_bucket }
    }

    fn handle_request(&self, num_tokens: usize, bucket_to_handle: &str) {
        if self.token_bucket.handle(num_tokens, bucket_to_handle).is_ok() {
            println!(
                "Request handled by service {} for configuration {}: {:?}",
                self.name,
                bucket_to_handle,
                thread::current().id()
            );
        } else {
            println!(
                "Rate limit exceeded by service {} for configuration {}: {:?}",
                self.name,
                bucket_to_handle,
                thread::current().id()
            );
        }
    }
}

#[test]
fn test_token_bucket() {
    let token_bucket = Arc::new(TokenBucket::new());

    // Seed the token bucket with predefined configurations
    let configs = [
        Configs::MESSAGES,
        Configs::CLIENT_CREATE,
        Configs::CLIENT_UPDATE,
        Configs::CLIENT_DELETE,
        Configs::USER_REGISTRATION,
        Configs::USER_CONFIRM_REGISTRATION,
        Configs::USER_LOGIN_ATTEMPT,
        Configs::USER_FORGOT_PASSWORD,
        Configs::USER_CHANGE_EMAIL,
        Configs::USER_DELETE_USER,
    ];
    token_bucket.seed_buckets(&configs);

    // Create service instances
    let service_a = Service::new("A".to_string(), token_bucket.clone());
    let service_b = Service::new("B".to_string(), token_bucket.clone());

    // Perform requests to the services
    for _ in 0..10 {
        // Handle requests for the "messages" service
        let service_a = service_a.clone();
        thread::spawn(move || {
            service_a.handle_request(1, Configs::MESSAGES.name);
        });

        // Handle requests for the "registration" service
        let service_b = service_b.clone();
        thread::spawn(move || {
            service_b.handle_request(1, Configs::USER_REGISTRATION.name);
        });

        // Sleep for a short duration between requests
        thread::sleep(Duration::from_millis(100));
    }

    // Sleep for a longer duration to allow token refilling
    thread::sleep(Duration::from_secs(2));

    // Perform more requests after token refilling
    for _ in 0..5 {
        // Handle requests for the "messages" service
        let service_a = service_a.clone();
        thread::spawn(move || {
            service_a.handle_request(1, Configs::MESSAGES.name);
        });

        // Handle requests for the "registration" service
        let service_b = service_b.clone();
        thread::spawn(move || {
            service_b.handle_request(1, Configs::USER_REGISTRATION.name);
        });

        // Sleep for a short duration between requests
        thread::sleep(Duration::from_millis(100));
    }
}
