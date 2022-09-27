use serde::{Deserialize, Serialize};

// SpotConfig is the 'spot' of the NetspotConfig
//--------------------------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct SpotConfig {
    #[serde(default = "spot_default_depth")]
    pub depth: i32,
    #[serde(default = "spot_default_q")]
    pub q: f64,
    #[serde(default = "spot_default_n_init")]
    pub n_init: u32,
    #[serde(default = "spot_default_level")]
    pub level: f64,
    #[serde(default = "spot_default_up")]
    pub up: bool,
    #[serde(default = "spot_default_down")]
    pub down: bool,
    #[serde(default = "spot_default_alert")]
    pub alert: bool,
    #[serde(default = "spot_default_bounded")]
    pub bounded: bool,
    #[serde(default = "spot_default_max_excess")]
    pub max_excess: u32,
}

// Default values
//--------------------------------------------------------------------------------------------------

impl Default for SpotConfig {
    fn default() -> Self {
        SpotConfig {
            depth: spot_default_depth(),
            q: spot_default_q(),
            n_init: spot_default_n_init(),
            level: spot_default_level(),
            up: spot_default_up(),
            down: spot_default_down(),
            alert: spot_default_alert(),
            bounded: spot_default_bounded(),
            max_excess: spot_default_max_excess(),
        }
    }
}

fn spot_default_depth() -> i32 {
    50
}

fn spot_default_q() -> f64 {
    1e-4
}

fn spot_default_n_init() -> u32 {
    1000
}

fn spot_default_level() -> f64 {
    0.8
}

fn spot_default_up() -> bool {
    true
}

fn spot_default_down() -> bool {
    false
}

fn spot_default_alert() -> bool {
    true
}

fn spot_default_bounded() -> bool {
    true
}

fn spot_default_max_excess() -> u32 {
    200
}

// Unit tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        // Deserialize should use default values for missing configs
        let config: SpotConfig = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(50, config.depth);
        assert_eq!(1e-4, config.q);
        assert_eq!(1000, config.n_init);
        assert_eq!(0.8, config.level);
        assert_eq!(true, config.up);
        assert_eq!(false, config.down);
        assert_eq!(true, config.alert);
        assert_eq!(true, config.bounded);
        assert_eq!(200, config.max_excess);
    }

    #[test]
    fn all_config() {
        // We should be able to modify all configs
        let config: SpotConfig = serde_json::from_str(
            r#"{
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
        assert_eq!(1, config.depth);
        assert_eq!(2.0, config.q);
        assert_eq!(3, config.n_init);
        assert_eq!(4.0, config.level);
        assert_eq!(false, config.up);
        assert_eq!(true, config.down);
        assert_eq!(false, config.alert);
        assert_eq!(false, config.bounded);
        assert_eq!(5, config.max_excess);
    }
}
