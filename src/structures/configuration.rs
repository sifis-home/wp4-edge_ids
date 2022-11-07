pub mod influxdb;
pub mod miner;
pub mod spot;
pub mod stats;

use crate::structures::configuration::influxdb::InfluxDB1Config;
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Default configuration JSON
//--------------------------------------------------------------------------------------------------
pub const DEFAULT_NETSPOT_CONFIG_JSON: &str = r#"{
	"configuration": {
		"name": "Default configuration"
	},
	"spot": {
		"depth": 50,
		"q": 0.00001,
		"n_init": 2000,
		"level": 0.98,
		"up": true,
		"down": false,
		"alert": true,
		"bounded": true,
		"max_excess": 200
	},
	"stats": {
		"avg_pkt_size": {
			"enabled": true,
			"max_excess": 1
		},
		"perf": {
			"enabled": true,
			"up": false
		},
		"r_arp": {
			"enabled": true
		},
		"r_syn": {
			"enabled": true
		},
		"traffic": {
			"enabled": true
		}
	}
}"#;

// NetspotConfig is used to generate config file for netspot process
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct NetspotConfig {
    pub configuration: miner::MinerConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influxdb1: Option<InfluxDB1Config>,
    #[serde(default)]
    pub spot: spot::SpotConfig,
    #[serde(default)]
    pub stats: stats::StatsConfig,
}

impl NetspotConfig {
    fn make_influxdb1_toml(&self) -> String {
        match self.influxdb1.as_ref() {
            None => String::from(""),
            Some(influxdb1) => format!(
                r#"
[exporter.influxdb]
data = {data}
alarm = {alarm}
address = "{address}"
database = "{database}"
username = "{username}"
password = "{password}"
batch_size = {batch_size}
agent_name = "{agent_name}"
"#,
                data = influxdb1.data,
                alarm = influxdb1.data,
                address = influxdb1.address,
                database = influxdb1.database,
                username = influxdb1.username,
                password = influxdb1.password,
                batch_size = influxdb1.batch_size,
                agent_name = influxdb1.agent_name
            ),
        }
    }

    pub fn make_toml(&self) -> String {
        format!(
            r#"[miner]
device = "{device}"
promiscuous = {promiscuous}
snapshot_len = 65535
timeout = "0s"

{analyzer}

[exporter.socket]
data = "unix:///tmp/netspot_data.socket"
alarm = "unix:///tmp/netspot_alarm.socket"
tag = "{tag}"
format = "json"
{influxdb1}
[spot]
depth = {depth}
q = {q}
n_init = {n_init}
level = {level}
up = {up}
down = {down}
alert = {alert}
bounded = {bounded}
max_excess = {max_excess}
{spot_overrides}"#,
            device = self.configuration.device,
            promiscuous = self.configuration.promiscuous,
            analyzer = self.stats.make_analyzer_toml(),
            tag = self.configuration.name,
            influxdb1 = self.make_influxdb1_toml(),
            depth = self.spot.depth,
            q = self.spot.q,
            n_init = self.spot.n_init,
            level = self.spot.level,
            up = self.spot.up,
            down = self.spot.down,
            alert = self.spot.alert,
            bounded = self.spot.bounded,
            max_excess = self.spot.max_excess,
            spot_overrides = self.stats.make_spots_toml()
        )
    }
}

impl Default for NetspotConfig {
    fn default() -> Self {
        serde_json::from_str(DEFAULT_NETSPOT_CONFIG_JSON).unwrap()
    }
}

// Id mapped to configuration
//--------------------------------------------------------------------------------------------------

pub type NetspotConfigMap = HashMap<i32, NetspotConfig>;

// Unit tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration() {
        // We should be able to deserialize default configuration from JSON
        let config: NetspotConfig = serde_json::from_str(DEFAULT_NETSPOT_CONFIG_JSON).unwrap();

        // MinerConfig
        assert_eq!("Default configuration", config.configuration.name);
        assert_eq!("any", config.configuration.device);
        assert!(config.configuration.promiscuous);
        assert!(config.configuration.enabled);

        // SpotConfig
        assert_eq!(50, config.spot.depth);
        assert_eq!(1e-5, config.spot.q);
        assert_eq!(2000, config.spot.n_init);
        assert_eq!(0.98, config.spot.level);
        assert!(config.spot.up);
        assert!(!config.spot.down);
        assert!(config.spot.alert);
        assert!(config.spot.bounded);
        assert_eq!(200, config.spot.max_excess);

        // StatsConfig
        if let Some(ref config) = config.stats.avg_pkt_size {
            assert!(config.enabled);
            assert_eq!(Some(1), config.max_excess);
        } else {
            panic!("We should have avg_pkt_size configuration here");
        }
        if let Some(ref config) = config.stats.perf {
            assert!(config.enabled);
            assert_eq!(Some(false), config.up);
        } else {
            panic!("We should have perf configuration here");
        }
        if let Some(ref config) = config.stats.r_arp {
            assert!(config.enabled);
        } else {
            panic!("We should have r_arp configuration here");
        }
        if let Some(ref config) = config.stats.r_syn {
            assert!(config.enabled);
        } else {
            panic!("We should have r_syn configuration here");
        }
        if let Some(ref config) = config.stats.traffic {
            assert!(config.enabled);
        } else {
            panic!("We should have traffic configuration here");
        }

        // Rest of stats config should be None
        assert!(config.stats.r_ack.is_none());
        assert!(config.stats.r_dst_src.is_none());
        assert!(config.stats.r_dst_src_port.is_none());
        assert!(config.stats.r_icmp.is_none());
        assert!(config.stats.r_ip.is_none());
    }

    #[test]
    fn name_only() {
        // The configuration.name is the only required field. Others have defaults.
        let config: NetspotConfig =
            serde_json::from_str(r#"{"configuration":{"name":"test"}}"#).unwrap();

        // MinerConfig
        assert_eq!("test", config.configuration.name);
        assert_eq!("any", config.configuration.device);
        assert!(config.configuration.promiscuous);
        assert!(config.configuration.enabled);

        // SpotConfig
        assert_eq!(50, config.spot.depth);
        assert_eq!(1e-4, config.spot.q);
        assert_eq!(1000, config.spot.n_init);
        assert_eq!(0.8, config.spot.level);
        assert!(config.spot.up);
        assert!(!config.spot.down);
        assert!(config.spot.alert);
        assert!(config.spot.bounded);
        assert_eq!(200, config.spot.max_excess);

        // All stats should be None
        assert_eq!(None, config.stats.avg_pkt_size);
        assert_eq!(None, config.stats.perf);
        assert_eq!(None, config.stats.r_ack);
        assert_eq!(None, config.stats.r_arp);
        assert_eq!(None, config.stats.r_dst_src);
        assert_eq!(None, config.stats.r_dst_src_port);
        assert_eq!(None, config.stats.r_icmp);
        assert_eq!(None, config.stats.r_ip);
        assert_eq!(None, config.stats.r_syn);
        assert_eq!(None, config.stats.traffic);
    }

    #[test]
    fn missing_required_fields() {
        // Deserialization should fail if the required configuration field is missing
        let result: Result<NetspotConfig, _> = serde_json::from_str("{}");
        assert!(result.is_err());

        // Deserialization should fail if required name field is missing from the configuration
        let result: Result<NetspotConfig, _> = serde_json::from_str(r#"{"configuration":{}}"#);
        assert!(result.is_err());
    }

    #[test]
    fn make_toml() {
        // Default configuration should make the following output
        let expected = r#"[miner]
device = "any"
promiscuous = true
snapshot_len = 65535
timeout = "0s"

[analyzer]
period = "1s"
stats = ["AVG_PKT_SIZE", "PERF", "R_ARP", "R_SYN", "TRAFFIC"]

[exporter.socket]
data = "unix:///tmp/netspot_data.socket"
alarm = "unix:///tmp/netspot_alarm.socket"
tag = "Default configuration"
format = "json"

[spot]
depth = 50
q = 0.00001
n_init = 2000
level = 0.98
up = true
down = false
alert = true
bounded = true
max_excess = 200

[spot.AVG_PKT_SIZE]
max_excess = 1

[spot.PERF]
up = false
"#;
        let config: NetspotConfig = serde_json::from_str(DEFAULT_NETSPOT_CONFIG_JSON).unwrap();
        assert_eq!(config.make_toml(), expected);
    }

    #[test]
    fn test_influxdb1_config() {
        let json = r#"{
	"configuration": {
		"name": "InfluxDB test"
	},
	"influxdb1": {
		"data": true,
		"alarm": true,
		"address": "http://host.docker.internal:8086",
		"database": "database_example",
		"username": "username_example",
		"password": "password_example",
		"batch_size": 5,
		"agent_name": "agent_example"
	}
}"#;
        let config = serde_json::from_str::<NetspotConfig>(json).unwrap();
        let influxdb1 = config.influxdb1.as_ref().unwrap();
        assert!(influxdb1.data);
        assert!(influxdb1.alarm);
        assert_eq!(influxdb1.address, "http://host.docker.internal:8086");
        assert_eq!(influxdb1.database, "database_example");
        assert_eq!(influxdb1.username, "username_example");
        assert_eq!(influxdb1.password, "password_example");
        assert_eq!(influxdb1.batch_size, 5);
        assert_eq!(influxdb1.agent_name, "agent_example");

        // Configuration should make the following TOML
        let expected = r#"[miner]
device = "any"
promiscuous = true
snapshot_len = 65535
timeout = "0s"

[analyzer]
period = "1s"
stats = []

[exporter.socket]
data = "unix:///tmp/netspot_data.socket"
alarm = "unix:///tmp/netspot_alarm.socket"
tag = "InfluxDB test"
format = "json"

[exporter.influxdb1]
data = true
alarm = true
address = "http://host.docker.internal:8086"
database = "database_example"
username = "username_example"
password = "password_example"
batch_size = 5
agent_name = "agent_example"

[spot]
depth = 50
q = 0.0001
n_init = 1000
level = 0.8
up = true
down = false
alert = true
bounded = true
max_excess = 200
"#;
        assert_eq!(config.make_toml(), expected);
    }
}
