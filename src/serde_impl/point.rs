use serde::de::Visitor;
use serde::{Deserialize, Deserializer, de};
use serde_json::from_str;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
struct Data {
    #[serde(deserialize_with = "de_point")]
    point: Point,
}

struct PointVisitor;

impl<'de> Visitor<'de> for PointVisitor {
    type Value = Point;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Point struct with x & y")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let st: Vec<&str> = v.split(",").collect();

        if st.len() != 2 {
            return Err(de::Error::custom("expect 2 points"));
        }
        let x = st[0]
            .trim()
            .parse()
            .map_err(|_| de::Error::custom("x is not a number"))?;
        let y = st[1]
            .trim()
            .parse()
            .map_err(|_| de::Error::custom("y is not a number"))?;

        Ok(Point { x, y })
    }
}

fn de_point<'de, D>(deserializer: D) -> Result<Point, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(PointVisitor)
}

#[test]
fn test_ok_simple() {
    let json = r#"{ "point": "10,20" }"#;
    let data: Data = from_str(json).unwrap();
    assert_eq!(data.point, Point { x: 10, y: 20 });
}

#[test]
fn test_ok_with_spaces() {
    let json = r#"{ "point": "10, 20" }"#;
    let data: Data = from_str(json).unwrap();
    assert_eq!(data.point, Point { x: 10, y: 20 });
}

#[test]
fn test_missing_second_value() {
    let json = r#"{ "point": "10" }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}

#[test]
fn test_too_many_values() {
    let json = r#"{ "point": "10,20,30" }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}

#[test]
fn test_not_numbers() {
    let json = r#"{ "point": "a,b" }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}

#[test]
fn test_number_instead_of_string() {
    let json = r#"{ "point": 42 }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}

#[test]
fn test_missing_field() {
    let json = r#"{ }"#;
    let res = from_str::<Data>(json);
    assert!(res.is_err());
}
