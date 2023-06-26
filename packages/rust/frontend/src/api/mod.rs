mod client;
mod message;
mod user;

#[derive(Clone)]
pub struct BubbleApi {
    domain: String,
    bearer: String,
    client: reqwest::Client,
}

impl BubbleApi {
    pub fn new(domain: String, bearer: String) -> Self {
        // create client with bearer header
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", bearer)).unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            domain,
            bearer,
            client,
        }
    }
}
