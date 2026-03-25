// deserialize with transformation with From

use std::result;

use csv::{Reader, ReaderBuilder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct RawUser {
    id: i32,
    full_name: String,
    phone: String,
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    surname: String,
    phone: String,
}

impl TryFrom<RawUser> for User {
    type Error = String;

    fn try_from(value: RawUser) -> Result<Self, Self::Error> {
        let (name, surname) = value
            .full_name
            .split_once(" ")
            .ok_or("expacted \"name surname\"")?;

        let mut phone = String::new();
        for c in value.phone.chars() {
            if c == '-' {
                continue;
            }
            if !c.is_ascii_digit() {
                return Err("phone must contain only digits".into());
            }
            phone.push(c);
        }

        if phone.is_empty() {
            return Err("empty phone".into());
        }

        Ok(User {
            id: value.id,
            name: name.to_string(),
            surname: surname.to_string(),
            phone,
        })
    }
}

#[test]
fn simple_test() {
    let csv_str = "1;Mike Richard;12-23-32";
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(csv_str.as_bytes());

    for result in rdr.deserialize::<RawUser>() {
        let raw = result.unwrap();

        let user = User::try_from(raw).unwrap();
        println!("{:?}", user);
    }

}
