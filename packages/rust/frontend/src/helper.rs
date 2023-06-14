use crate::GLOBAL_ACCOUNT_DATA;

pub async fn account_url(path: &str) -> String {
    let domain = GLOBAL_ACCOUNT_DATA
        .read()
        .await
        .as_ref()
        .unwrap()
        .domain
        .clone();
    format!("https://{}{}", domain, path)
}
