use serde::{Deserialize, Serialize};

// Stat is configuration for named stats
//--------------------------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StatConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_init: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub up: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub down: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_excess: Option<u32>,
}

// Default values
//--------------------------------------------------------------------------------------------------

impl Default for StatConfig {
    fn default() -> Self {
        StatConfig {
            enabled: false,
            depth: None,
            q: None,
            n_init: None,
            level: None,
            up: None,
            down: None,
            alert: None,
            bounded: None,
            max_excess: None,
        }
    }
}

// Unit tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        // Deserialize should use default values for missing configs
        let config: StatConfig = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(false, config.enabled);
        assert_eq!(None, config.depth);
        assert_eq!(None, config.q);
        assert_eq!(None, config.n_init);
        assert_eq!(None, config.level);
        assert_eq!(None, config.up);
        assert_eq!(None, config.down);
        assert_eq!(None, config.alert);
        assert_eq!(None, config.bounded);
        assert_eq!(None, config.max_excess);
    }

    #[test]
    fn all_config() {
        // We should be able to modify all configs
        let config: StatConfig = serde_json::from_str(
            r#"{
        "enabled": true,
        "depth": 1,
        "q": 2.0,
        "n_init": 3,
        "level": 4.0,
        "up": false,
        "down": true,
        "alert": false,
        "bounded": false,
        "max_excess": 5
    }"#,
        )
        .unwrap();
        assert_eq!(true, config.enabled);
        assert_eq!(Some(1), config.depth);
        assert_eq!(Some(2.0), config.q);
        assert_eq!(Some(3), config.n_init);
        assert_eq!(Some(4.0), config.level);
        assert_eq!(Some(false), config.up);
        assert_eq!(Some(true), config.down);
        assert_eq!(Some(false), config.alert);
        assert_eq!(Some(false), config.bounded);
        assert_eq!(Some(5), config.max_excess);
    }
}
