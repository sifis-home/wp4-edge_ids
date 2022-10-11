pub mod stat;

use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};

// Stats contains which statistics are calculated. All stats are optional.
//--------------------------------------------------------------------------------------------------
#[derive(Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub struct StatsConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_pkt_size: Option<stat::StatConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perf: Option<stat::StatConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_ack: Option<stat::StatConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_arp: Option<stat::StatConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_dst_src: Option<stat::StatConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_dst_src_port: Option<stat::StatConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_icmp: Option<stat::StatConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_ip: Option<stat::StatConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_syn: Option<stat::StatConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic: Option<stat::StatConfig>,
}

impl StatsConfig {
    pub fn make_analyzer_toml(&self) -> String {
        // Array of stats with their names
        let stats = [
            ("AVG_PKT_SIZE", &self.avg_pkt_size),
            ("PERF", &self.perf),
            ("R_ACK", &self.r_ack),
            ("R_ARP", &self.r_arp),
            ("R_DST_SRC", &self.r_dst_src),
            ("R_DST_SRC_PORT", &self.r_dst_src_port),
            ("R_ICMP", &self.r_icmp),
            ("R_IP", &self.r_ip),
            ("R_SYN", &self.r_syn),
            ("TRAFFIC", &self.traffic),
        ];
        // Finding enabled stats
        let mut enabled_stats = Vec::new();
        for (name, stat) in stats {
            if let Some(config) = stat {
                if config.enabled {
                    enabled_stats.push(name);
                }
            }
        }
        // Making string presentation of enabled stats
        let stats = {
            if enabled_stats.is_empty() {
                "[]".to_string()
            } else {
                format!("[\"{}\"]", enabled_stats.join("\", \""))
            }
        };
        // Making analyzer configuration in TOML
        format!("[analyzer]\nperiod = \"1s\"\nstats = {stats}")
    }

    pub fn make_spots_toml(&self) -> String {
        // Array of stats with their spot configuration names
        let stats = [
            ("spot.AVG_PKT_SIZE", &self.avg_pkt_size),
            ("spot.PERF", &self.perf),
            ("spot.R_ACK", &self.r_ack),
            ("spot.R_ARP", &self.r_arp),
            ("spot.R_DST_SRC", &self.r_dst_src),
            ("spot.R_DST_SRC_PORT", &self.r_dst_src_port),
            ("spot.R_ICMP", &self.r_icmp),
            ("spot.R_IP", &self.r_ip),
            ("spot.R_SYN", &self.r_syn),
            ("spot.TRAFFIC", &self.traffic),
        ];
        // Making spot configs for enabled stats with settings overrides
        let mut result = String::new();
        for (name, stat) in stats {
            if let Some(config) = stat {
                let toml = config.make_toml();
                if !toml.is_empty() {
                    if !result.is_empty() {
                        result.push('\n');
                    }
                    result.push_str(format!("[{}]\n{}", name, toml).as_str());
                }
            }
        }
        // Return result string
        result
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
        let config: StatsConfig = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(None, config.avg_pkt_size);
        assert_eq!(None, config.perf);
        assert_eq!(None, config.r_ack);
        assert_eq!(None, config.r_arp);
        assert_eq!(None, config.r_dst_src);
        assert_eq!(None, config.r_dst_src_port);
        assert_eq!(None, config.r_icmp);
        assert_eq!(None, config.r_ip);
        assert_eq!(None, config.r_syn);
        assert_eq!(None, config.traffic);
    }

    #[test]
    fn some_configs() {
        // If we can modify settings for one of the statistics, then we
        // can do the same for others as they all use same StatConfig struct
        let config: StatsConfig = serde_json::from_str(
            r#"{
                "perf": {
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
            }
           }"#,
        )
        .unwrap();

        // We should have perf config now
        assert!(config.perf.is_some());
        if let Some(ref perf) = config.perf {
            assert!(perf.enabled);
            assert_eq!(Some(1), perf.depth);
            assert_eq!(Some(2.0), perf.q);
            assert_eq!(Some(3), perf.n_init);
            assert_eq!(Some(4.0), perf.level);
            assert_eq!(Some(false), perf.up);
            assert_eq!(Some(true), perf.down);
            assert_eq!(Some(false), perf.alert);
            assert_eq!(Some(false), perf.bounded);
            assert_eq!(Some(5), perf.max_excess);
        }

        // Other configs should be None
        assert_eq!(None, config.avg_pkt_size);
        assert_eq!(None, config.r_ack);
        assert_eq!(None, config.r_arp);
        assert_eq!(None, config.r_dst_src);
        assert_eq!(None, config.r_dst_src_port);
        assert_eq!(None, config.r_icmp);
        assert_eq!(None, config.r_ip);
        assert_eq!(None, config.r_syn);
        assert_eq!(None, config.traffic);
    }

    #[test]
    fn empty_stats() {
        // We should get empty stats list, disabled stats should not be visible in the list
        let config: StatsConfig = serde_json::from_str(
            r#"{
        "avg_pkt_size":{"enabled":false},
        "perf":{"enabled":false,"q":1.0}
        }"#,
        )
        .unwrap();
        let expected = r#"[analyzer]
period = "1s"
stats = []"#;
        assert_eq!(config.make_analyzer_toml(), expected);

        // We should get empty listing for spot settings
        let spots_toml = config.make_spots_toml();
        assert!(spots_toml.is_empty());
    }

    #[test]
    fn full_stats() {
        // We should get list of all stats for the analyzer
        let config: StatsConfig = serde_json::from_str(
            r#"{
        "avg_pkt_size":{"enabled":true,"q":1.1},
        "perf":{"enabled":true,"q":1.1},
        "r_ack":{"enabled":true,"q":1.1},
        "r_arp":{"enabled":true,"q":1.1},
        "r_dst_src":{"enabled":true,"q":1.1},
        "r_dst_src_port":{"enabled":true,"q":1.1},
        "r_icmp":{"enabled":true,"q":1.1},
        "r_ip":{"enabled":true,"q":1.1},
        "r_syn":{"enabled":true,"q":1.1},
        "traffic":{"enabled":true,"q":1.1}
        }"#,
        )
        .unwrap();
        let expected = r#"[analyzer]
period = "1s"
stats = ["AVG_PKT_SIZE", "PERF", "R_ACK", "R_ARP", "R_DST_SRC", "R_DST_SRC_PORT", "R_ICMP", "R_IP", "R_SYN", "TRAFFIC"]"#;
        assert_eq!(config.make_analyzer_toml(), expected);

        // We should get SPOT overrides for all items where q is 1
        let expected = r#"[spot.AVG_PKT_SIZE]
q = 1.1

[spot.PERF]
q = 1.1

[spot.R_ACK]
q = 1.1

[spot.R_ARP]
q = 1.1

[spot.R_DST_SRC]
q = 1.1

[spot.R_DST_SRC_PORT]
q = 1.1

[spot.R_ICMP]
q = 1.1

[spot.R_IP]
q = 1.1

[spot.R_SYN]
q = 1.1

[spot.TRAFFIC]
q = 1.1
"#;
        assert_eq!(config.make_spots_toml(), expected);
    }
}
