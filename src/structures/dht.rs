use crate::structures::statistics::AlarmMessage;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtMessage {
    pub request_post_topic_uuid: RequestPostTopicUUID,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestPostTopicUUID {
    pub topic_name: String,
    pub topic_uuid: String,
    pub value: Value,
}

impl RequestPostTopicUUID {
    pub fn new(addresses: &[String], alarm: AlarmMessage) -> Self {
        RequestPostTopicUUID {
            topic_name: "SIFIS:Netspot_Alarm".to_string(),
            topic_uuid: "Netspot_Alarm".to_string(),
            value: Value::new(addresses, alarm),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Value {
    pub description: String,
    pub addresses: Vec<String>,
    pub alarm: AlarmMessage,
}

impl Value {
    pub fn new(addresses: &[String], alarm: AlarmMessage) -> Self {
        Value {
            description: "Netspot Anomaly Alarm".to_string(),
            addresses: addresses.to_owned(),
            alarm,
        }
    }
}
