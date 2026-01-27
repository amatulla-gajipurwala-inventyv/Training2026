use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    zipcode: String,
}

impl Address {
    // Constructor
    fn new(street: String, city: String, zipcode: String) -> Self {
        Self {
            street,
            city,
            zipcode,
        }
    }

    // Getters
    fn street(&self) -> &str {
        &self.street
    }

    fn city(&self) -> &str {
        &self.city
    }

    fn zipcode(&self) -> &str {
        &self.zipcode
    }

    fn full_address(&self) -> String {
        format!("{}, {}, {}", self.street, self.city, self.zipcode)
    }

    // Setters
    fn set_street(&mut self, street: String) {
        self.street = street;
    }

    fn set_city(&mut self, city: String) {
        self.city = city;
    }

    fn set_zipcode(&mut self, zipcode: String) {
        self.zipcode = zipcode;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    id: u32,
    name: String,
    address: Address,
    phone: String,
    email: String,
}

impl Employee {
    // Constructor
    fn new(id: u32, name: String, address: Address, phone: String, email: String) -> Self {
        Self {
            id,
            name,
            address,
            phone,
            email,
        }
    }

    // Getters
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &Address {
        &self.address
    }

    fn phone(&self) -> &str {
        &self.phone
    }

    fn email(&self) -> &str {
        &self.email
    }

    // Other methods
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }

    fn contact_info(&self) -> String {
        format!("{} | {} | {}", self.name, self.phone, self.email)
    }

    // Setters
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_address(&mut self, address: Address) {
        self.address = address;
    }

    fn set_phone(&mut self, phone: String) {
        self.phone = phone;
    }

    fn set_email(&mut self, email: String) {
        self.email = email;
    }

    // Nested updates
    fn update_city(&mut self, city: String) {
        self.address.set_city(city);
    }

    fn update_street(&mut self, street: String) {
        self.address.set_street(street);
    }

    fn update_zipcode(&mut self, zipcode: String) {
        self.address.set_zipcode(zipcode);
    }

    
    fn info_with_args(
        &self,
        id: u32,
        name: &str,
        street: &str,
        city: &str,
        zipcode: &str,
        phone: &str,
        email: &str,
    ) -> String {
        format!(
            "ID: {}, Name: {}, Address: {}, {}, {}, Phone: {}, Email: {}",
            id, name, street, city, zipcode, phone, email
        )
    }
}

fn main() {
  
    let home = Address::new(
        String::from("123 Main St"),
        String::from("Amd"),
        String::from("110001"),
    );

    
    let mut employee = Employee::new(
        1,
        String::from("Amatulla"),
        home,
        String::from("123-456-7890"),
        String::from("amatulla@gmail.com"),
    );

    println!("Employee Greeting: {}", employee.greet());
    println!("Employee Contact: {}", employee.contact_info());
    println!(
        "Employee Full Address: {}",
        employee.address().full_address()
    );

    // Update entire address
    let new_address = Address::new(
        String::from("789 Park"),
        String::from("Vadodara"),
        String::from("560001"),
    );
    employee.set_address(new_address);

   

    // With-args method usage
    let info = employee.info_with_args(
        employee.id(),
        employee.name(),
        employee.address().street(),
        employee.address().city(),
        employee.address().zipcode(),
        employee.phone(),
        employee.email(),
    );

    println!("\nInfo with args:");
    println!("{}", info);

    // Update nested fields
    employee.update_city(String::from("Amd"));
    employee.update_street(String::from("456 New Street"));
    employee.update_zipcode(String::from("400001"));

    employee.set_phone(String::from("999-888-7777"));
    employee.set_email(String::from("amatulla12@gmail.com"));

    println!("\nAfter updates:");
    println!(
        "Employee Full Address: {}",
        employee.address().full_address()
    );
    println!("Employee Contact: {}", employee.contact_info());

    // SERIALIZATION 
    let json = serde_json::to_string_pretty(&employee).unwrap();
    println!("\nSerialized Employee (JSON):");
    println!("{}", json);

    //  DESERIALIZATION
    let employee_from_json: Employee = serde_json::from_str(&json).unwrap();

    println!("\nDeserialized Employee Struct:");
    println!("{:?}", employee_from_json);
}
