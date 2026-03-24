use serde::de::Visitor;
use serde::{Deserialize, Deserializer, de};
use serde_json::from_str;

#[derive(Debug, PartialEq)]
enum Status {
    Ok,
    Error,
}

#[derive(Deserialize)]
struct Response {
    #[serde(deserialize_with = "de_status")]
    status: Status,
}

struct StatusVisitor;

impl<'de> Visitor<'de> for StatusVisitor {
    type Value = Status;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Status enum")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v.trim().to_lowercase().as_str() {
            "ok" => Ok(Status::Ok),
            "error" => Ok(Status::Error),
            _ => return Err(de::Error::custom("invalid input - wrong status")),
        }
    }
}

fn de_status<'de, D>(deserializer: D) -> Result<Status, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(StatusVisitor)
}

#[test]
fn test_ok_lowercase() {
    let json = r#"{ "status": "ok" }"#;
    let res: Response = from_str(json).unwrap();
    assert_eq!(res.status, Status::Ok);
}

#[test]
fn test_ok_uppercase() {
    let json = r#"{ "status": "OK" }"#;
    let res: Response = from_str(json).unwrap();
    assert_eq!(res.status, Status::Ok);
}

#[test]
fn test_error_mixed_case() {
    let json = r#"{ "status": "ErRoR" }"#;
    let res: Response = from_str(json).unwrap();
    assert_eq!(res.status, Status::Error);
}

#[test]
fn test_unknown_value() {
    let json = r#"{ "status": "unknown" }"#;
    let res = from_str::<Response>(json);
    assert!(res.is_err());
}

#[test]
fn test_number_instead_of_string() {
    let json = r#"{ "status": 42 }"#;
    let res = from_str::<Response>(json);
    assert!(res.is_err());
}

#[test]
fn test_missing_field() {
    let json = r#"{ }"#;
    let res = from_str::<Response>(json);
    assert!(res.is_err());
}
