use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Webhook related enums and types
//--------------------------------------------------------------------------------------------------

pub type WebhookHeaders = HashMap<String, String>;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum WebhookRequestMethod {
    Get,
    #[default]
    Post,
    Put,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WebhookStatsType {
    Alarms, // Only alarms
    #[default]
    Both, // Both alarms and data
    Data,   // Only data
}

// Webhook
//--------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Webhook {
    pub name: String,
    pub address: String,
    #[serde(default)]
    pub method: WebhookRequestMethod,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: WebhookHeaders,
    #[serde(default, rename = "type")]
    pub stats_type: WebhookStatsType,
}

// Webhook listing
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct WebhookItem {
    pub id: i32,
    pub name: String,
}

pub type WebhookList = Vec<WebhookItem>;

// Container for webhook configurations
//--------------------------------------------------------------------------------------------------

pub type Webhooks = HashMap<i32, Webhook>;

// Unit tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers() {
        let json = r#"{"key1":"value1","key2":"value2","key3":"value3"}"#;
        let headers = serde_json::from_str::<WebhookHeaders>(json).unwrap();
        let expected = WebhookHeaders::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]);
        assert_eq!(headers, expected);
    }

    #[test]
    fn method() {
        let methods = vec![
            WebhookRequestMethod::Get,
            WebhookRequestMethod::Post,
            WebhookRequestMethod::Put,
        ];
        let json = serde_json::to_string(&methods).unwrap();
        let expected = r#"["GET","POST","PUT"]"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn stats_type() {
        let stats_types = vec![
            WebhookStatsType::Alarms,
            WebhookStatsType::Both,
            WebhookStatsType::Data,
        ];
        let json = serde_json::to_string(&stats_types).unwrap();
        let expected = r#"["alarms","both","data"]"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn webhook() {
        let hook = Webhook {
            name: "test".to_string(),
            address: "http://captain.hook/".to_string(),
            method: WebhookRequestMethod::Post,
            headers: HashMap::from([("code".to_string(), "12345".to_string())]),
            stats_type: WebhookStatsType::Both,
        };
        let json = serde_json::to_string(&hook).unwrap();
        let expected = concat!(
            r#"{"name":"test","address":"http://captain.hook/","#,
            r#""method":"POST","headers":{"code":"12345"},"type":"both"}"#
        );
        assert_eq!(json, expected);

        let json = r#"{
	"name": "Captain Hook",
	"address": "https://captain.hook/",
	"method": "GET",
	"headers": {
		"key": "12345",
		"gold": "1991"
	},
	"type": "data"
}"#;
        let hook = serde_json::from_str::<Webhook>(json).unwrap();
        let expected = Webhook {
            name: "Captain Hook".to_string(),
            address: "https://captain.hook/".to_string(),
            method: WebhookRequestMethod::Get,
            headers: WebhookHeaders::from([
                ("key".to_string(), "12345".to_string()),
                ("gold".to_string(), "1991".to_string()),
            ]),
            stats_type: WebhookStatsType::Data,
        };
        assert_eq!(hook, expected);
    }

    #[test]
    fn webhooks() {
        let hooks: WebhookList = vec![
            WebhookItem {
                id: 1,
                name: "Captain Hook".to_string(),
            },
            WebhookItem {
                id: 2,
                name: "Example".to_string(),
            },
        ];
        let json = serde_json::to_string(&hooks).unwrap();
        let expected = r#"[{"id":1,"name":"Captain Hook"},{"id":2,"name":"Example"}]"#;
        assert_eq!(json, expected);

        let json = r#"[
  {
    "id": 1,
    "name": "My example callback #1"
  },
  {
    "id": 2,
    "name": "My example callback #2"
  }
]"#;
        let hooks = serde_json::from_str::<WebhookList>(json).unwrap();
        let expected = vec![
            WebhookItem {
                id: 1,
                name: "My example callback #1".to_string(),
            },
            WebhookItem {
                id: 2,
                name: "My example callback #2".to_string(),
            },
        ];
        assert_eq!(hooks, expected);
    }

    #[test]
    fn defaults() {
        // Only name and address are required. Others should have the following defaults:
        // method: POST
        // headers: (empty)
        // type: both
        let json = r#"{"name":"test","address":"test"}"#;
        let hook = serde_json::from_str::<Webhook>(json).unwrap();
        let expected = Webhook {
            name: "test".to_string(),
            address: "test".to_string(),
            method: WebhookRequestMethod::Post,
            headers: Default::default(),
            stats_type: WebhookStatsType::Both,
        };
        assert_eq!(hook, expected);
    }

    #[test]
    fn skip_empty_headers() {
        let hook = Webhook {
            name: "name".to_string(),
            address: "address".to_string(),
            method: Default::default(),
            headers: Default::default(),
            stats_type: Default::default(),
        };
        let json = serde_json::to_string(&hook).unwrap();
        let expected = r#"{"name":"name","address":"address","method":"POST","type":"both"}"#;
        // Note: expected JSON should not have headers field at all.
        assert_eq!(json, expected);
    }
}
