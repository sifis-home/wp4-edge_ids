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

impl StatConfig {
    pub fn make_toml(&self) -> String {
        let mut output = String::new();
        // We give output only for enabled stats
        if !self.enabled {
            return output;
        }
        if let Some(value) = self.depth {
            output.push_str(format!("depth = {}\n", value).as_str());
        };
        if let Some(value) = self.q {
            output.push_str(format!("q = {}\n", value).as_str());
        };
        if let Some(value) = self.n_init {
            output.push_str(format!("n_init = {}\n", value).as_str());
        };
        if let Some(value) = self.level {
            output.push_str(format!("level = {}\n", value).as_str());
        };
        if let Some(value) = self.up {
            output.push_str(format!("up = {}\n", value).as_str());
        };
        if let Some(value) = self.down {
            output.push_str(format!("down = {}\n", value).as_str());
        };
        if let Some(value) = self.alert {
            output.push_str(format!("alert = {}\n", value).as_str());
        };
        if let Some(value) = self.bounded {
            output.push_str(format!("bounded = {}\n", value).as_str());
        };
        if let Some(value) = self.max_excess {
            output.push_str(format!("max_excess = {}\n", value).as_str());
        };
        output
    }
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

        // TOML output should be empty
        assert!(config.make_toml().is_empty());
    }

    #[test]
    fn all_config() {
        // We should be able to modify all configs
        let config: StatConfig = serde_json::from_str(
            r#"{
        "enabled": true,
        "depth": 1,
        "q": 2.2,
        "n_init": 3,
        "level": 4.4,
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
        assert_eq!(Some(2.2), config.q);
        assert_eq!(Some(3), config.n_init);
        assert_eq!(Some(4.4), config.level);
        assert_eq!(Some(false), config.up);
        assert_eq!(Some(true), config.down);
        assert_eq!(Some(false), config.alert);
        assert_eq!(Some(false), config.bounded);
        assert_eq!(Some(5), config.max_excess);

        // Checking also TOML output
        let expected = "depth = 1\n\
        q = 2.2\n\
        n_init = 3\n\
        level = 4.4\n\
        up = false\n\
        down = true\n\
        alert = false\n\
        bounded = false\n\
        max_excess = 5\n";
        assert_eq!(config.make_toml(), expected)
    }
}
