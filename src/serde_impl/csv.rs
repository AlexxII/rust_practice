use std::result;

use serde::de::Visitor;
use serde::{Deserialize, de};

#[derive(PartialEq, Debug)]
struct User {
    id: u32,
    name: String,
    surname: String,
    phone: String,
}

struct UserVisitor;

impl<'de> Visitor<'de> for UserVisitor {
    type Value = User;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "User struct - User id, name, surname, phone")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let id: u32 = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

        let fio: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;

        let (name, surname) = fio
            .split_once(" ")
            .ok_or_else(|| de::Error::custom("expected format \"name surname\""))?;

        let phone = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;

        if seq.next_element::<de::IgnoredAny>()?.is_some() {
            return Err(de::Error::custom("too many fields"));
        }

        Ok(User {
            id,
            name: name.to_string(),
            surname: surname.to_string(),
            phone,
        })
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(UserVisitor)
    }
}

#[test]
fn easy_test() {
    let csv_str = r#"1;San Paulo;23-43-53"#;

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(csv_str.as_bytes());
    // for result in rdr.records() {
    //     let record = result.unwrap();
    //     println!("{:?}", record);
    // }

    for result in rdr.deserialize::<User>() {
        let record = result.unwrap();
        println!("{:?}", record);
    }
}
