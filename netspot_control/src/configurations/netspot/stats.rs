pub mod stat;

use serde::{Deserialize, Serialize};

// Stats contains which statistics are calculated. All stats are optional.
//--------------------------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StatsConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_pkg_size: Option<stat::StatConfig>,
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

// Default values
//--------------------------------------------------------------------------------------------------

impl Default for StatsConfig {
    fn default() -> Self {
        StatsConfig {
            avg_pkg_size: None,
            perf: None,
            r_ack: None,
            r_arp: None,
            r_dst_src: None,
            r_dst_src_port: None,
            r_icmp: None,
            r_ip: None,
            r_syn: None,
            traffic: None,
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
        let config: StatsConfig = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(None, config.avg_pkg_size);
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
            assert_eq!(true, perf.enabled);
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
        assert_eq!(None, config.avg_pkg_size);
        assert_eq!(None, config.r_ack);
        assert_eq!(None, config.r_arp);
        assert_eq!(None, config.r_dst_src);
        assert_eq!(None, config.r_dst_src_port);
        assert_eq!(None, config.r_icmp);
        assert_eq!(None, config.r_ip);
        assert_eq!(None, config.r_syn);
        assert_eq!(None, config.traffic);
    }
}
