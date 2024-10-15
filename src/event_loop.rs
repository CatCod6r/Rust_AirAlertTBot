use crate::api_connector;
use reqwest::header::HeaderValue;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;
use teloxide::prelude::*;

pub async fn run_alert_loop(
    chat_id: &ChatId,
    bot: Bot,
    header_value: HeaderValue,
    should_stop: &bool,
)
{
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    let mut last_alert: bool = false;
    loop {
        println!("Should it stop? {}", should_stop);
        if *should_stop {
            break;
        }
        interval.tick().await;
        let current_alert = api_connector::recieve_json(header_value.clone())
            .await
            .unwrap();
        println!("{:?}", current_alert); //Log info
        if !current_alert.is_empty() {
            if !current_alert.first().unwrap().is_null() && !last_alert {
                for alert in handle_alerts(&current_alert).await.unwrap() {
                    bot.send_message(*chat_id, alert).await.unwrap();
                }
                last_alert = true;
            }
        } else {
            if last_alert {
                bot.send_message(*chat_id, "Відбій тривоги :D")
                    .await
                    .unwrap();
            }
            last_alert = false;
        }
    }
}
pub async fn handle_alerts(current_alert: &[Value]) -> Result<Vec<String>, Box<dyn Error>> {
    let mut vec_to_return: Vec<String> = Vec::new();
    for alert_type in current_alert {
        match alert_type.get("type").unwrap().as_str().unwrap() {
            "AIR" => vec_to_return.push("Повітряна тривога".to_string()),
            "ARTILLERY" => vec_to_return.push("Загроза артилерії".to_string()),
            "URBAN_FIGHTS" => vec_to_return.push("Вуличні бої".to_string()),
            "CHEMICAL" => vec_to_return.push("Загроза хімічної зброї".to_string()),
            "NUCLEAR" => vec_to_return.push("Ну все хана ядерка".to_string()),
            _ => {}
        }
    }
    Ok(vec_to_return)
}

