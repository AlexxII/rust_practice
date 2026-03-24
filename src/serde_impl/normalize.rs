use serde::{Deserialize, Deserializer, de};
use serde_json::from_str;

#[derive(Deserialize)]
struct User {
    #[serde(deserialize_with = "de_phone")]
    phone: String,
}

fn de_phone<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Err(de::Error::custom("empty string"));
    }
    let s = s.replace("-", "");
    if s.chars().all(|c| c.is_ascii_digit()) {
        return Ok(s);
    } else {
        return Err(de::Error::custom("not a number"));
    }
}

#[test]
fn test_ok_with_dashes() {
    let json = r#"{ "phone": "23-43-23" }"#;
    let user: User = from_str(json).unwrap();
    assert_eq!(user.phone, "234323");
}

#[test]
fn test_ok_no_dashes() {
    let json = r#"{ "phone": "123456" }"#;
    let user: User = from_str(json).unwrap();
    assert_eq!(user.phone, "123456");
}

#[test]
fn test_invalid_characters() {
    let json = r#"{ "phone": "12-3a" }"#;
    let res = from_str::<User>(json);
    assert!(res.is_err());
}

#[test]
fn test_number_instead_of_string() {
    let json = r#"{ "phone": 123456 }"#;
    let res = from_str::<User>(json);
    assert!(res.is_err());
}

#[test]
fn test_missing_field() {
    let json = r#"{ }"#;
    let res = from_str::<User>(json);
    assert!(res.is_err());
}

#[test]
fn test_empty_string() {
    let json = r#"{ "phone": "" }"#;
    let res = from_str::<User>(json);
    assert!(res.is_err());
}
