use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};

// MinerConfig is the 'configuration' of the NetspotConfig
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct MinerConfig {
    // Name is required field
    pub name: String,
    #[serde(default = "miner_default_device")]
    pub device: String,
    #[serde(default = "miner_default_promiscuous")]
    pub promiscuous: bool,
    #[serde(default = "miner_default_enabled")]
    pub enabled: bool,
}

// Default values
//--------------------------------------------------------------------------------------------------

fn miner_default_device() -> String {
    "any".to_string()
}

fn miner_default_promiscuous() -> bool {
    true
}

fn miner_default_enabled() -> bool {
    true
}

// Unit tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        // Deserialize should use default values for missing configs
        let config: MinerConfig = serde_json::from_str(r#"{"name":"test"}"#).unwrap();
        assert_eq!("test", config.name);
        assert_eq!("any", config.device);
        assert!(config.promiscuous);
        assert!(config.enabled);
    }

    #[test]
    fn all_config() {
        // We should be able to modify all configs
        let config: MinerConfig = serde_json::from_str(
            r#"{"name":"test","device":"eth0","promiscuous":false,"enabled": false}"#,
        )
        .unwrap();
        assert_eq!("test", config.name);
        assert_eq!("eth0", config.device);
        assert!(!config.promiscuous);
        assert!(!config.enabled);
    }

    #[test]
    fn missing_name() {
        // Following should fail as the required name is missing
        let result: Result<MinerConfig, _> = serde_json::from_str(r#"{"device":"eth0"}"#);
        assert!(result.is_err());
    }
}
