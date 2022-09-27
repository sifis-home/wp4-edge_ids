pub mod miner;
pub mod spot;
pub mod stats;

use serde::{Deserialize, Serialize};

// NetspotConfig is used to generate config file for netspot process
//--------------------------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Debug)]
pub struct NetspotConfig {
    pub configuration: miner::MinerConfig,
    #[serde(default)]
    pub spot: spot::SpotConfig,
    #[serde(default)]
    pub stats: stats::StatsConfig,
}

// Unit tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_CONFIG_JSON: &str = r#"{
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
		"avg_pkg_size": {
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

    #[test]
    fn default_configuration() {
        // We should be able to deserialize default configuration from JSON
        let config: NetspotConfig = serde_json::from_str(DEFAULT_CONFIG_JSON).unwrap();

        // MinerConfig
        assert_eq!("Default configuration", config.configuration.name);
        assert_eq!("any", config.configuration.device);
        assert_eq!(true, config.configuration.promiscuous);
        assert_eq!(true, config.configuration.enabled);

        // SpotConfig
        assert_eq!(50, config.spot.depth);
        assert_eq!(1e-5, config.spot.q);
        assert_eq!(2000, config.spot.n_init);
        assert_eq!(0.98, config.spot.level);
        assert_eq!(true, config.spot.up);
        assert_eq!(false, config.spot.down);
        assert_eq!(true, config.spot.alert);
        assert_eq!(true, config.spot.bounded);
        assert_eq!(200, config.spot.max_excess);

        // StatsConfig
        if let Some(ref config) = config.stats.avg_pkg_size {
            assert_eq!(true, config.enabled);
            assert_eq!(Some(1), config.max_excess);
        } else {
            panic!("We should have avg_pkg_size configuration here");
        }
        if let Some(ref config) = config.stats.perf {
            assert_eq!(true, config.enabled);
            assert_eq!(Some(false), config.up);
        } else {
            panic!("We should have perf configuration here");
        }
        if let Some(ref config) = config.stats.r_arp {
            assert_eq!(true, config.enabled);
        } else {
            panic!("We should have r_arp configuration here");
        }
        if let Some(ref config) = config.stats.r_syn {
            assert_eq!(true, config.enabled);
        } else {
            panic!("We should have r_syn configuration here");
        }
        if let Some(ref config) = config.stats.traffic {
            assert_eq!(true, config.enabled);
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
        assert_eq!(true, config.configuration.promiscuous);
        assert_eq!(true, config.configuration.enabled);

        // SpotConfig
        assert_eq!(50, config.spot.depth);
        assert_eq!(1e-4, config.spot.q);
        assert_eq!(1000, config.spot.n_init);
        assert_eq!(0.8, config.spot.level);
        assert_eq!(true, config.spot.up);
        assert_eq!(false, config.spot.down);
        assert_eq!(true, config.spot.alert);
        assert_eq!(true, config.spot.bounded);
        assert_eq!(200, config.spot.max_excess);

        // All stats should be None
        assert_eq!(None, config.stats.avg_pkg_size);
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
}
