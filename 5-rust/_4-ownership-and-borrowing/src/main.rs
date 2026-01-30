#[derive(Debug)]
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

    // // Getters
    // fn street(&self) -> &str {
    //     &self.street
    // }

    // fn city(&self) -> &str {
    //     &self.city
    // }

    // fn zipcode(&self) -> &str {
    //     &self.zipcode
    // }

    // fn full_address(&self) -> String {
    //     format!("{}, {}, {}", self.street, self.city, self.zipcode)
    // }

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

#[derive(Debug)]
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

    // // Getters
    // fn id(&self) -> u32 {
    //     self.id
    // }

    // fn name(&self) -> &str {
    //     &self.name
    // }

    // fn address(&self) -> &Address {
    //     &self.address
    // }

    // fn phone(&self) -> &str {
    //     &self.phone
    // }

    // fn email(&self) -> &str {
    //     &self.email
    // }

    // // Other methods
    // fn greet(&self) -> String {
    //     format!("Hello, {}!", self.name)
    // }

    // fn contact_info(&self) -> String {
    //     format!("{} | {} | {}", self.name, self.phone, self.email)
    // }

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

    // // Nested updates
    fn update_city(&mut self, city: String) {
        self.address.set_city(city);
    }

    // fn update_street(&mut self, street: String) {
    //     self.address.set_street(street);
    // }

    // fn update_zipcode(&mut self, zipcode: String) {
    //     self.address.set_zipcode(zipcode);
    // }

    fn print(&self) {
        println!("{:#?}", self);
    }

    // fn info_with_args(
    //     &self,
    //     id: u32,
    //     name: &str,
    //     street: &str,
    //     city: &str,
    //     zipcode: &str,
    //     phone: &str,
    //     email: &str,
    // ) -> String {
    //     format!(
    //         "ID: {}, Name: {}, Address: {}, {}, {}, Phone: {}, Email: {}",
    //         id, name, street, city, zipcode, phone, email
    //     )
    // }
}

fn main() {
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

    println!("--- Before change ---");
    employee.print();

    
        let emp_ref: &mut Employee = &mut employee;
        emp_ref.set_name("AG".to_string());
        println!("\nAfter name update (via ref):");
        emp_ref.print();
    
    println!("Printing using original struct:");
    employee.print();

    
        let emp_ref: &mut Employee = &mut employee;
        emp_ref.update_city("Mumbai".to_string());
        println!("\nAfter city update (via ref):");
        emp_ref.print();
    
    println!("Printing using original struct:");
    employee.print();

    
        let emp_ref: &mut Employee = &mut employee;
        emp_ref.set_phone("8888888888".to_string());
        println!("\nAfter phone update (via ref):");
        emp_ref.print();
    

    
    println!("Printing using original struct:");
    employee.print();
    //  println!("\nPrinting using mutable reference:");
    // emp_ref.print();
}
