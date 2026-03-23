use serde::Deserialize;
use serde::de::{self, Visitor};
use serde_json::from_str;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

struct UserVisitor {}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    Id,
    Name,
}

impl<'de> Visitor<'de> for UserVisitor {
    type Value = User;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "User struct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut id = None;
        let mut name = None;

        while let Some(key) = map.next_key::<Field>()? {
            match key {
                Field::Id => {
                    id = map.next_value()?;
                }
                Field::Name => {
                    name = map.next_value()?;
                }
            }
        }
        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;

        Ok(User { id, name })
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(UserVisitor {})
    }
}

fn main() {
    let json_str = r#"{"id": 123, "name": "Sheeeet"}"#;
    let user = from_str::<User>(json_str);
    if user.is_ok() {
        println!("{:?}", user.unwrap());
    } else {
        println!("{:?}", user.err());
    }
}
