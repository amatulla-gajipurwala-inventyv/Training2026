#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    zipcode: String,
}

impl Address {
    fn new(street: String, city: String, zipcode: String) -> Self {
        Self { street, city, zipcode }
    }

    fn set_city(&mut self, city: String) {
        self.city = city;
    }
}

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
    address: Address,
    phone: String,
    email: String,
}

impl Employee {
    fn new(id: u32, name: String, address: Address, phone: String, email: String) -> Self {
        Self { id, name, address, phone, email }
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}

pub fn run() {
    let address = Address::new(
        "AG Road".to_string(),
        "Amd".to_string(),
        "560001".to_string(),
    );

    let mut employee = Employee::new(
        1,
        "Amatulla".to_string(),
        address,
        "9999999999".to_string(),
        "amatulla@example.com".to_string(),
    );

    employee.print();
    employee.address.set_city("Mumbai".to_string());
    employee.print();
}
