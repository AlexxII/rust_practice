use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "PascalCase")]
struct Dog {
    name: String,
    year_born: u16,
    owner: Owner,
}

#[derive(Serialize, Deserialize, Debug)]
struct Owner {
    first_name: String,
    last_name: String,
}

fn main() {
    let owner = Owner {
        first_name: "Trawor".to_string(),
        last_name: "Sheeet".to_string(),
    };
    let dog = Dog {
        name: "Some sheeet".to_string(),
        year_born: 2022,
        owner,
    };
    let dog_json = to_string_pretty(&dog);
    if dog_json.is_ok() {
        println!("{}", dog_json.ok().unwrap());
    } else {
        println!("{:?}", dog_json.err());
    }

    deserialize();
}

fn deserialize() {
    let json_string: &str = r#"
        {
          "name": "Some sheeet",
          "year_born": 2022,
          "owner": {
            "first_name": "Trawor",
            "last_name": "Sheeet"
          }
        } 
    "#;
    let dog = from_str::<Dog>(json_string);
    if dog.is_ok() {
        println!("{:#?}", dog.unwrap());
    } else {
        println!("{:?}", dog.err())
    }
}
