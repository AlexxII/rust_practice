use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::from_str;

#[derive(Deserialize)]
struct Data {
    #[serde(deserialize_with = "de_value")]
    value: i32,
}

struct I32Visitor;

impl<'de> Visitor<'de> for I32Visitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "i32 in string or number")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let v = i32::try_from(v).map_err(|_| de::Error::custom("out of range"))?;
        Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v = i32::try_from(v).map_err(|_| de::Error::custom("out of range"))?;
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let x = v
            .trim()
            .parse()
            .map_err(|_| de::Error::custom("value is not a valid number"))?;
        Ok(x)
    }
}

fn de_value<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(I32Visitor)
}

#[test]
fn test_string_ok() {
    let json = r#"{ "value": "10" }"#;
    let data: Data = from_str(json).unwrap();
    assert_eq!(data.value, 10);
}

#[test]
fn test_number_ok() {
    let json = r#"{ "value": 10 }"#;
    let data: Data = from_str(json).unwrap();
    assert_eq!(data.value, 10);
}

#[test]
fn test_invalid_string() {
    let json = r#"{ "value": "abc" }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}

#[test]
fn test_bool_error() {
    let json = r#"{ "value": true }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}

#[test]
fn test_object_error() {
    let json = r#"{ "value": {} }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}

#[test]
fn test_array_error() {
    let json = r#"{ "value": [] }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}

#[test]
fn test_value_empty() {
    let json = r#"{ "value":  }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}

#[test]
fn test_missing_field() {
    let json = r#"{ }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}
