use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};

// InfluxDB1Config is the 'influxdb1' of the NetspotConfig
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct InfluxDB1Config {
    #[serde(default = "influx1_default_data")]
    pub data: bool,
    #[serde(default = "influx1_default_alarm")]
    pub alarm: bool,
    #[serde(default = "influx1_default_address")]
    pub address: String,
    #[serde(default = "influx1_default_database")]
    pub database: String,
    #[serde(default = "influx1_default_username")]
    pub username: String,
    #[serde(default = "influx1_default_password")]
    pub password: String,
    #[serde(default = "influx1_default_batch_size")]
    pub batch_size: i32,
    #[serde(default = "influx1_default_agent_name")]
    pub agent_name: String,
}

// Default values
//--------------------------------------------------------------------------------------------------

impl Default for InfluxDB1Config {
    fn default() -> InfluxDB1Config {
        InfluxDB1Config {
            data: influx1_default_data(),
            alarm: influx1_default_alarm(),
            address: influx1_default_address(),
            database: influx1_default_database(),
            username: influx1_default_username(),
            password: influx1_default_password(),
            batch_size: influx1_default_batch_size(),
            agent_name: influx1_default_agent_name(),
        }
    }
}

fn influx1_default_data() -> bool {
    false
}

fn influx1_default_alarm() -> bool {
    false
}

fn influx1_default_address() -> String {
    String::from("http://127.0.0.1:8086")
}

fn influx1_default_database() -> String {
    String::from("netspot")
}

fn influx1_default_username() -> String {
    String::from("netspot")
}

fn influx1_default_password() -> String {
    String::from("netspot")
}

fn influx1_default_batch_size() -> i32 {
    10
}

fn influx1_default_agent_name() -> String {
    String::from("local")
}

// Unit tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        // Deserialize should use default values for missing configs
        let config = serde_json::from_str::<InfluxDB1Config>(r#"{}"#).unwrap();
        assert!(!config.data);
        assert!(!config.alarm);
        assert_eq!(config.address, "http://127.0.0.1:8086");
        assert_eq!(config.database, "netspot");
        assert_eq!(config.username, "netspot");
        assert_eq!(config.password, "netspot");
        assert_eq!(config.batch_size, 10);
        assert_eq!(config.agent_name, "local");
    }

    #[test]
    fn test_serialize() {
        let config = InfluxDB1Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let expected = concat!(
            r#"{"data":false,"alarm":false,"address":"http://127.0.0.1:8086","#,
            r#""database":"netspot","username":"netspot","password":"netspot","#,
            r#""batch_size":10,"agent_name":"local"}"#
        );
        assert_eq!(json, expected);
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
	"data": true,
	"alarm": true,
	"address": "http://host.docker.internal:8086",
	"database": "database_example",
	"username": "username_example",
	"password": "password_example",
	"batch_size": 5,
	"agent_name": "agent_example"
}"#;
        let config = serde_json::from_str::<InfluxDB1Config>(json).unwrap();
        assert!(config.data);
        assert!(config.alarm);
        assert_eq!(config.address, "http://host.docker.internal:8086");
        assert_eq!(config.database, "database_example");
        assert_eq!(config.username, "username_example");
        assert_eq!(config.password, "password_example");
        assert_eq!(config.batch_size, 5);
        assert_eq!(config.agent_name, "agent_example");
    }
}
