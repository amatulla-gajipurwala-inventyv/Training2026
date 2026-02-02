#[derive(Debug)]
struct Employee {
    name: String,
}

impl Employee {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn print(&self) {
        println!("{:?}", self);
    }
}

pub fn run() {
    let mut emp = Employee {
        name: "Amatulla".to_string(),
    };

    let emp_ref: &mut Employee = &mut emp;
    emp_ref.set_name("AG".to_string());
    emp_ref.print();

    emp.print();
}
