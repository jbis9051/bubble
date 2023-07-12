mod client;
mod message;
mod user;

#[derive(Clone)]
pub struct BubbleApi {
    domain: String,
    bearer: Option<String>,
    client: reqwest::Client,
}

impl BubbleApi {
    pub fn new(domain: String, bearer: Option<String>) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(bearer) = &bearer {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", bearer)).unwrap(),
            );
        }

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
