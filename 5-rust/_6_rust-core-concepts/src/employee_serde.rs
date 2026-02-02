use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    zipcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    id: u32,
    name: String,
    address: Address,
    phone: String,
    email: String,
}

pub fn run() {
    let employee = Employee {
        id: 1,
        name: "Amatulla".into(),
        address: Address {
            street: "123 Main St".into(),
            city: "Amd".into(),
            zipcode: "110001".into(),
        },
        phone: "999-888-7777".into(),
        email: "amatulla@gmail.com".into(),
    };

    let json = serde_json::to_string_pretty(&employee).unwrap();
    println!("{}", json);

    let back: Employee = serde_json::from_str(&json).unwrap();
    println!("{:?}", back);
}
