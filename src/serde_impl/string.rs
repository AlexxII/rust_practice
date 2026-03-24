use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    #[serde(deserialize_with = "de_id")]
    id: i32,
}

struct IdVisitor;

impl<'de> Visitor<'de> for IdVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "id - i32")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse()
            .map_err(|_| de::Error::custom("invalid - piece of sheet"))
    }
}

fn de_id<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(IdVisitor)
}

#[test]
fn test_ok() {
    let json_str = r#"{"id": "42"}"#;
    let user: User = from_str(&json_str).unwrap();
    assert_eq!(user.id, 42);
}

#[test]
fn test_invalid_number() {
    let json = r#"{ "id": "abc" }"#;
    let user = serde_json::from_str::<User>(json);
    assert!(user.is_err());
}

#[test]
fn test_number_instead_of_string() {
    let json = r#"{ "id": 42 }"#;
    let user = serde_json::from_str::<User>(json);
    assert!(user.is_err());
}

#[test]
fn test_missing_field() {
    let json = r#"{ }"#;
    let user = serde_json::from_str::<User>(json);
    assert!(user.is_err());
}

#[test]
fn test_error_message() {
    let json = r#"{ "id": "abc" }"#;
    let err = serde_json::from_str::<User>(json).unwrap_err();
    println!("{}", err);
}
