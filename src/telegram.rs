use serde_json::json;
use serde_json::{Map, Value};
use ureq;

use crate::errors::Result;

pub fn send_message(msg: String, token: &str, chat_id: String) -> Result<ureq::Response> {
    let mut request_body = Map::new();
    request_body.insert("text".to_string(), Value::String(msg));
    request_body.insert("chat_id".to_string(), Value::String(chat_id));
    request_body.insert(
        "parse_mode".to_string(),
        Value::String("MarkdownV2".to_string()),
    );
    request_body.insert("disable_web_page_preview".to_string(), Value::Bool(true));

    Ok(ureq::post(&format!(
        "https://api.telegram.org/bot{token}/sendMessage",
        token = &token
    ))
    .send_json(json!(request_body))?)
}
