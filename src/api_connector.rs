use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use serde_json::Value;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct RegionData {
    activeAlerts: Vec<Value>,
}

pub async fn recieve_json(header_value: HeaderValue) -> Result<Vec<Value>, Box<dyn Error>> {
    let url = "https://api.ukrainealarm.com/api/v3/alerts/31";
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, header_value);

    let response = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let parsed: Vec<RegionData> = serde_json::from_str(&response.text().await.unwrap()).expect("Failed to parse JSON");
        let active_alerts = parsed[0].activeAlerts.clone();
        Ok(active_alerts)
    } else {
        Ok(vec![Value::Null])
    }
}

