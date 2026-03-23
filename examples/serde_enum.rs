use serde::Deserialize;
use serde::de::{self, Visitor};

#[derive(Debug)]
enum Command {
    Ping,
    Echo(String),
    Add { a: i32, b: i32 },
}

struct CommandVisitor {}

#[derive(Deserialize)]
struct AddData {
    a: i32,
    b: i32,
}

impl<'de> Visitor<'de> for CommandVisitor {
    type Value = Command;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string or a map representing a Command")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        const VARIANTS: &[&str] = &["Ping", "Echo", "Add"];

        match v {
            "Ping" => Ok(Command::Ping),
            _ => Err(de::Error::unknown_variant(v, VARIANTS)),
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let key = match map.next_key()? {
            Some(k) => k,
            None => return Err(de::Error::custom("expected a command variant")),
        };
        let result = match key {
            "Ping" => {
                let _: de::IgnoredAny = map.next_value()?;
                Command::Ping
            }
            "Echo" => {
                let s: String = map.next_value()?;
                Command::Echo(s)
            }
            "Add" => {
                let data: AddData = map.next_value()?;
                Command::Add {
                    a: data.a,
                    b: data.b,
                }
            }
            _ => {
                return Err(de::Error::unknown_variant(key, &["Ping", "Echo", "Add"]));
            }
        };
        if map.next_key::<de::IgnoredAny>()?.is_some() {
            return Err(de::Error::custom("expected exactly one key"));
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(CommandVisitor {})
    }
}

fn main() {
    use serde_json::from_str;

    let cases = vec![
        // --- OK cases ---
        (r#""Ping""#, true),
        (r#"{ "Echo": "hello" }"#, true),
        (r#"{ "Add": { "a": 1, "b": 2 } }"#, true),
        // --- Errors ---
        (r#"{ }"#, false),       // пустой объект
        (r#""Unknown""#, false), // неизвестный variant
        (r#"{ "Unknown": 123 }"#, false),
        (r#"{ "Echo": 123 }"#, false),                // неверный тип
        (r#"{ "Add": { "a": 1 } }"#, false),          // missing field
        (r#"{ "Echo": "hi", "Ping": null }"#, false), // лишний ключ
    ];

    for (input, should_ok) in cases {
        let res = from_str::<Command>(input);

        match (res, should_ok) {
            (Ok(v), true) => println!("OK   {:?} -> {:?}", input, v),
            (Err(e), false) => println!("ERR  {:?} -> {:?}", input, e),
            (Ok(v), false) => println!("FAIL {:?} -> unexpected OK: {:?}", input, v),
            (Err(e), true) => println!("FAIL {:?} -> unexpected ERR: {:?}", input, e),
        }
    }
}
