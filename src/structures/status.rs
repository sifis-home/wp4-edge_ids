use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};

// Process status
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ProcessStatus {
    Running,
    Stopped,
    Disabled,
}

// Netspot status structure
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct Status {
    pub id: i32,
    pub name: String,
    pub status: ProcessStatus,
}

// Collection of statuses are simply vector of Status structures
//--------------------------------------------------------------------------------------------------

pub type Statuses = Vec<Status>;

// Unit tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_deserialize() {
        let status =
            serde_json::from_str::<Status>(r#"{"id":1,"name":"Test","status":"running"}"#).unwrap();
        assert_eq!(status.id, 1);
        assert_eq!(status.name, "Test");
        assert_eq!(status.status, ProcessStatus::Running);
    }

    #[test]
    fn status_serialize() {
        let status = Status {
            id: 1,
            name: "Test".to_string(),
            status: ProcessStatus::Running,
        };
        let json = serde_json::to_string(&status).unwrap();
        let expected = r#"{"id":1,"name":"Test","status":"running"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn statuses_deserialize() {
        let statuses = serde_json::from_str::<Statuses>(concat!(
            r#"[{"id":1,"name":"Test","status":"running"},"#,
            r#"{"id":2,"name":"Another test","status":"stopped"},"#,
            r#"{"id":3,"name":"Yet another test","status":"disabled"}]"#
        ))
        .unwrap();

        assert_eq!(statuses[0].id, 1);
        assert_eq!(statuses[0].name, "Test");
        assert_eq!(statuses[0].status, ProcessStatus::Running);
        assert_eq!(statuses[1].id, 2);
        assert_eq!(statuses[1].name, "Another test");
        assert_eq!(statuses[1].status, ProcessStatus::Stopped);
        assert_eq!(statuses[2].id, 3);
        assert_eq!(statuses[2].name, "Yet another test");
        assert_eq!(statuses[2].status, ProcessStatus::Disabled);
    }

    #[test]
    fn statuses_serialize() {
        let statuses = vec![
            Status {
                id: 1,
                name: "Test".to_string(),
                status: ProcessStatus::Running,
            },
            Status {
                id: 2,
                name: "Another test".to_string(),
                status: ProcessStatus::Stopped,
            },
            Status {
                id: 3,
                name: "Yet another test".to_string(),
                status: ProcessStatus::Disabled,
            },
        ];
        let json = serde_json::to_string(&statuses).unwrap();
        let expected = concat!(
            r#"[{"id":1,"name":"Test","status":"running"},"#,
            r#"{"id":2,"name":"Another test","status":"stopped"},"#,
            r#"{"id":3,"name":"Yet another test","status":"disabled"}]"#
        );
        assert_eq!(json, expected);
    }
}
