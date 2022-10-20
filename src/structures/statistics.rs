use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    #[default]
    Alarm,
    Data,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Stat {
    #[default]
    AvgPktSize,
    Perf,
    RAck,
    RArp,
    RDstSrc,
    RDstSrcPort,
    RIcmp,
    RIp,
    RSyn,
    Traffic,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertStatus {
    #[default]
    DownAlert,
    UpAlert,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AlarmMessage {
    pub time: i64,
    pub name: String,
    pub series: String,
    pub stat: Stat,
    pub status: AlertStatus,
    pub value: f64,
    pub probability: f64,
    pub code: i32,
    #[serde(rename = "type")]
    pub msg_type: MessageType,
}

pub type AlarmMessages = Vec<AlarmMessage>;

#[derive(Clone, Debug, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DataMessage {
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "series")]
    pub series: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_pkt_size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_pkt_size_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_pkt_size_up: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perf: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perf_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perf_up: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_ack: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_ack_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_ack_up: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_arp: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_arp_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_arp_up: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_dst_src: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_dst_src_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_dst_src_up: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_dst_src_port: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_dst_src_port_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_dst_src_port_up: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_icmp: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_icmp_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_icmp_up: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_ip: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_ip_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_ip_up: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_syn: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_syn_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r_syn_up: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic_down: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic_up: Option<f64>,
    #[serde(rename = "type")]
    pub msg_type: MessageType,
}

pub type DataMessages = Vec<DataMessage>;

#[derive(Clone, Debug)]
pub enum Message {
    Alarm(Box<AlarmMessage>),
    Data(Box<DataMessage>),
}

impl Message {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        match self {
            Message::Alarm(value) => serde_json::to_string(value),
            Message::Data(value) => serde_json::to_string(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_types() {
        // These should be presented as a lowercase string
        let test = vec![MessageType::Alarm, MessageType::Data];
        let json = serde_json::to_string(&test).unwrap();
        let expected = r#"["alarm","data"]"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn stats() {
        // These should be presented as a screaming snake case string
        let test = vec![
            Stat::AvgPktSize,
            Stat::Perf,
            Stat::RAck,
            Stat::RArp,
            Stat::RDstSrc,
            Stat::RDstSrcPort,
            Stat::RIcmp,
            Stat::RIp,
            Stat::RSyn,
            Stat::Traffic,
        ];
        let json = serde_json::to_string(&test).unwrap();
        let expected = concat!(
            r#"["AVG_PKT_SIZE","PERF","R_ACK","R_ARP","R_DST_SRC","#,
            r#""R_DST_SRC_PORT","R_ICMP","R_IP","R_SYN","TRAFFIC"]"#
        );
        assert_eq!(json, expected);
    }

    #[test]
    fn status() {
        // These should be presented as a screaming snake case string
        let test = vec![AlertStatus::DownAlert, AlertStatus::UpAlert];
        let json = serde_json::to_string(&test).unwrap();
        let expected = r#"["DOWN_ALERT","UP_ALERT"]"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn alarm_message_deserialize() {
        let alarm = serde_json::from_str::<AlarmMessage>(concat!(
            r#"{"#,
            r#"    "code": 1,"#,
            r#"    "name": "Example","#,
            r#"    "probability": 0,"#,
            r#"    "series": "any-Oct-18-09:18:16.505","#,
            r#"    "stat": "R_ICMP","#,
            r#"    "status": "UP_ALERT","#,
            r#"    "time": 1666074152545768954,"#,
            r#"    "type": "alarm","#,
            r#"    "value": 0.5"#,
            r#"}"#
        ))
        .unwrap();

        assert_eq!(alarm.time, 1666074152545768954);
        assert_eq!(alarm.name, "Example");
        assert_eq!(alarm.series, "any-Oct-18-09:18:16.505");
        assert_eq!(alarm.stat, Stat::RIcmp);
        assert_eq!(alarm.status, AlertStatus::UpAlert);
        assert_eq!(alarm.value, 0.5);
        assert_eq!(alarm.probability, 0.0);
        assert_eq!(alarm.code, 1);
        assert_eq!(alarm.msg_type, MessageType::Alarm);
    }

    #[test]
    fn alarm_message_serialize() {
        let alarm = AlarmMessage {
            time: 1,
            name: "Example".to_string(),
            series: "Series".to_string(),
            stat: Stat::AvgPktSize,
            status: AlertStatus::DownAlert,
            value: 1.0,
            probability: 0.5,
            code: 1,
            msg_type: MessageType::Alarm,
        };
        let json = serde_json::to_string(&alarm).unwrap();
        let expected = concat!(
            r#"{"#,
            r#""time":1,"#,
            r#""name":"Example","#,
            r#""series":"Series","#,
            r#""stat":"AVG_PKT_SIZE","#,
            r#""status":"DOWN_ALERT","#,
            r#""value":1.0,"#,
            r#""probability":0.5,"#,
            r#""code":1,"#,
            r#""type":"alarm""#,
            r#"}"#
        );
        assert_eq!(json, expected);
    }

    #[test]
    fn data_message_deserialize() {
        let data = serde_json::from_str::<DataMessage>(concat!(
            r#"{"#,
            r#"    "AVG_PKT_SIZE": 1,"#,
            r#"    "AVG_PKT_SIZE_DOWN": 2,"#,
            r#"    "AVG_PKT_SIZE_UP": 3,"#,
            r#"    "PERF": 4,"#,
            r#"    "R_ACK": 5,"#,
            r#"    "R_ACK_DOWN": 6,"#,
            r#"    "R_ACK_UP": 7,"#,
            r#"    "R_ARP": 8,"#,
            r#"    "R_ARP_DOWN": 9,"#,
            r#"    "R_ARP_UP": 10,"#,
            r#"    "R_DST_SRC": 11,"#,
            r#"    "R_DST_SRC_DOWN": 12,"#,
            r#"    "R_DST_SRC_UP": 13,"#,
            r#"    "R_DST_SRC_PORT": 14,"#,
            r#"    "R_DST_SRC_PORT_DOWN": 15,"#,
            r#"    "R_DST_SRC_PORT_UP": 16,"#,
            r#"    "R_ICMP": 17,"#,
            r#"    "R_ICMP_DOWN": 18,"#,
            r#"    "R_ICMP_UP": 19,"#,
            r#"    "R_IP": 20,"#,
            r#"    "R_IP_DOWN": 21,"#,
            r#"    "R_IP_UP": 22,"#,
            r#"    "R_SYN": 23,"#,
            r#"    "R_SYN_DOWN": 24,"#,
            r#"    "R_SYN_UP": 25,"#,
            r#"    "TRAFFIC": 26,"#,
            r#"    "TRAFFIC_DOWN": 27,"#,
            r#"    "TRAFFIC_UP": 28,"#,
            r#"    "name": "TestName","#,
            r#"    "series": "TestSeries","#,
            r#"    "time": 29,"#,
            r#"    "type": "data""#,
            r#"}"#,
        ))
        .unwrap();
        assert!(data.avg_pkt_size.is_some());
        assert!(data.avg_pkt_size_down.is_some());
        assert!(data.avg_pkt_size_up.is_some());
        assert!(data.perf.is_some());
        assert!(data.perf_down.is_none());
        assert!(data.perf_up.is_none());
        assert!(data.r_ack.is_some());
        assert!(data.r_ack_down.is_some());
        assert!(data.r_ack_up.is_some());
        assert!(data.r_arp.is_some());
        assert!(data.r_arp_down.is_some());
        assert!(data.r_arp_up.is_some());
        assert!(data.r_dst_src.is_some());
        assert!(data.r_dst_src_down.is_some());
        assert!(data.r_dst_src_up.is_some());
        assert!(data.r_dst_src_port.is_some());
        assert!(data.r_dst_src_port_down.is_some());
        assert!(data.r_dst_src_port_up.is_some());
        assert!(data.r_icmp.is_some());
        assert!(data.r_icmp_down.is_some());
        assert!(data.r_icmp_up.is_some());
        assert!(data.r_ip.is_some());
        assert!(data.r_ip_down.is_some());
        assert!(data.r_ip_up.is_some());
        assert!(data.r_syn.is_some());
        assert!(data.r_syn_down.is_some());
        assert!(data.r_syn_up.is_some());
        assert!(data.traffic.is_some());
        assert!(data.traffic_down.is_some());
        assert!(data.traffic_up.is_some());
        assert_eq!(data.avg_pkt_size.unwrap(), 1.0);
        assert_eq!(data.avg_pkt_size_down.unwrap(), 2.0);
        assert_eq!(data.avg_pkt_size_up.unwrap(), 3.0);
        assert_eq!(data.perf.unwrap(), 4.0);
        assert_eq!(data.r_ack.unwrap(), 5.0);
        assert_eq!(data.r_ack_down.unwrap(), 6.0);
        assert_eq!(data.r_ack_up.unwrap(), 7.0);
        assert_eq!(data.r_arp.unwrap(), 8.0);
        assert_eq!(data.r_arp_down.unwrap(), 9.0);
        assert_eq!(data.r_arp_up.unwrap(), 10.0);
        assert_eq!(data.r_dst_src.unwrap(), 11.0);
        assert_eq!(data.r_dst_src_down.unwrap(), 12.0);
        assert_eq!(data.r_dst_src_up.unwrap(), 13.0);
        assert_eq!(data.r_dst_src_port.unwrap(), 14.0);
        assert_eq!(data.r_dst_src_port_down.unwrap(), 15.0);
        assert_eq!(data.r_dst_src_port_up.unwrap(), 16.0);
        assert_eq!(data.r_icmp.unwrap(), 17.0);
        assert_eq!(data.r_icmp_down.unwrap(), 18.0);
        assert_eq!(data.r_icmp_up.unwrap(), 19.0);
        assert_eq!(data.r_ip.unwrap(), 20.0);
        assert_eq!(data.r_ip_down.unwrap(), 21.0);
        assert_eq!(data.r_ip_up.unwrap(), 22.0);
        assert_eq!(data.r_syn.unwrap(), 23.0);
        assert_eq!(data.r_syn_down.unwrap(), 24.0);
        assert_eq!(data.r_syn_up.unwrap(), 25.0);
        assert_eq!(data.traffic.unwrap(), 26.0);
        assert_eq!(data.traffic_down.unwrap(), 27.0);
        assert_eq!(data.traffic_up.unwrap(), 28.0);
        assert_eq!(data.name, "TestName");
        assert_eq!(data.series, "TestSeries");
        assert_eq!(data.time, 29);
        assert_eq!(data.msg_type, MessageType::Data);
    }

    #[test]
    fn message_to_json() {
        let message = Message::Alarm(Box::new(AlarmMessage {
            time: 1,
            name: "AlarmName".to_string(),
            series: "AlarmSeries".to_string(),
            stat: Stat::AvgPktSize,
            status: AlertStatus::UpAlert,
            value: 2.0,
            probability: 3.0,
            code: 4,
            msg_type: MessageType::Alarm,
        }));
        let json = message.to_json().unwrap();
        let expected = concat!(
            r#"{"time":1,"name":"AlarmName","series":"AlarmSeries","stat":"AVG_PKT_SIZE","#,
            r#""status":"UP_ALERT","value":2.0,"probability":3.0,"code":4,"type":"alarm"}"#
        );
        assert_eq!(json, expected);

        let message = Message::Data(Box::new(DataMessage {
            time: 1,
            name: "DataName".to_string(),
            series: "DataSeries".to_string(),
            msg_type: MessageType::Data,
            ..DataMessage::default()
        }));
        let json = message.to_json().unwrap();
        let expected = r#"{"time":1,"name":"DataName","series":"DataSeries","type":"data"}"#;
        assert_eq!(json, expected);
    }
}
