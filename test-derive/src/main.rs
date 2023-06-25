use std::fmt::Display;

use json_derive::Json;
use json_trait::Json;

#[derive(Json)]
struct Person {
    name: String,
    age: i32,
    is_student: bool,
    address: Option<Address>,
    hobbies: Option<Vec<String>>,
    food: Food,
    friends: Vec<Person>,
}

#[derive(Json)]
struct Address {
    street: String,
    city: String,
    country: String,
}
impl Json for Food {
    fn to_json(&self) -> String {
        format!("{}", &self)
    }
}
#[derive(Json)]
enum Food {
    Chicken,
    Ham,
    Bacon,
}
impl Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Food::Bacon => write!(f, "Bacon"),
            Food::Chicken => write!(f, "Chicken"),
            Food::Ham => write!(f, "Ham"),
        }
    }
}
fn main() {
    let addr1 = Address {
        street: "123 Main St".into(),
        city: "Exampleville".into(),
        country: "Sampleland".into(),
    };
    let addr2 = Address {
        street: "Maple Street".into(),
        city: "Springfield".into(),
        country: "United States".into(),
    };
    let addr3 = Address {
        street: "123 Random S".into(),
        city: "Cityville".into(),
        country: "Countryland".into(),
    };
    let addr4 = Address {
        street: "Wonderland".into(),
        city: "Springfield".into(),
        country: "456 Elm St".into(),
    };
    let addr5 = Address {
        street: "456 Elm St".into(),
        city: "Randomville".into(),
        country: "Wonderland".into(),
    };
    println!("{}", addr1.to_json());
    println!("{}", addr2.to_json());
    println!("{}", addr3.to_json());
    println!("{}", addr4.to_json());
    println!("{}", addr5.to_json());
    let kofi = Person {
        name: "Kofi".into(),
        age: 21,
        is_student: true,
        address: Some(addr1),
        hobbies: Some(["hiking", "coding", "sleeping"].map(|h| h.into()).to_vec()),
        food: Food::Chicken,
        friends: vec![
            Person {
                name: "Jane Smith".into(),
                age: 28,
                is_student: false,
                address: Some(addr2),
                hobbies: None,
                food: Food::Ham,
                friends: vec![],
            },
            Person {
                name: "David Johnson".into(),
                age: 19,
                is_student: true,
                address: None,
                hobbies: None,
                food: Food::Chicken,
                friends: vec![],
            },
            Person {
                name: "John Doe".into(),
                age: 94,
                is_student: true,
                address: Some(addr5),
                hobbies: Some(vec!["being lazy".into()]),
                food: Food::Bacon,
                friends: vec![],
            }
        ],
    };
    println!("{}", kofi.to_json())
}
