use core::fmt;
use std::collections::{HashMap, hash_map::Entry};

trait DisplayItem {
    fn display(&self) -> String;
}

#[derive(Debug)]
enum InventoryError {
    DuplicateId(String),
    InvalidId,
    ItemNotFound(String),
}
impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::DuplicateId(id) => {
                write!(f, "Item with id '{}' already exist", id)
            }
            InventoryError::InvalidId => {
                write!(f, "Invalid Id")
            }
            InventoryError::ItemNotFound(id) => {
                write!(f, "Item with id '{}' not found", id)
            }
        }
    }
}
struct Inventory<T>
where
    T: DisplayItem + Clone,
{
    items: HashMap<String, T>,
}

impl<T> Inventory<T>
where
    T: DisplayItem + Clone,
{
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }
    fn add_item(&mut self, id: impl Into<String>, item: T) -> Result<(), InventoryError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }
        match self.items.entry(id) {
            Entry::Occupied(e) => Err(InventoryError::DuplicateId(e.key().clone())),
            Entry::Vacant(e) => {
                e.insert(item);
                Ok(())
            }
        }
    }
    fn get_item(&self, id: &str) -> Result<&T, InventoryError> {
        self.items
            .get(id)
            .ok_or_else(|| InventoryError::ItemNotFound(id.to_string()))
    }
    fn remove_item(&mut self, id: &str) -> Result<T, InventoryError> {
        self.items
            .remove(id)
            .ok_or_else(|| InventoryError::ItemNotFound(id.to_string()))
    }
    fn display_all(&self) -> String {
        if self.items.is_empty() {
            return "Inventory is empty".to_string();
        }
        self.items
            .iter()
            .map(|(id, item)| format!("Id: {} \n{}\n", id, item.display()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Clone)]
struct Product {
    name: String,
    price: f64,
}
impl DisplayItem for Product {
    fn display(&self) -> String {
        format!("Product: {}, Price:{}", self.name, self.price)
    }
}
fn main() {
    let mut inventory = Inventory::<Product>::new();
    println!("\nDiaplay all : {}", inventory.display_all());

    let items = [
        (
            "P1",
            Product {
                name: "Laptop".to_string(),
                price: 60000.0,
            },
        ),
        (
            "P2",
            Product {
                name: "Neckband".to_string(),
                price: 1500.0,
            },
        ),
        (
            "P1",
            Product {
                name: "PowerBank".to_string(),
                price: 1000.0,
            },
        ),
        (
            "",
            Product {
                name: "k".to_string(),
                price: 1000.0,
            },
        ),
    ];

    for (id, item) in items {
        if let Err(e) = inventory.add_item(id, item) {
            println!("Add failed: {}", e);
        }
    }

    println!(" \nCurrent Inventory :\n {}", inventory.display_all());

    match inventory.get_item("P2") {
        Ok(item) => println!("Fetched item: {}\n", item.display()),

        Err(e) => println!("Error: {}", e),
    }
     match inventory.get_item("P3") {
        Ok(item) => println!("Fetched item: {}\n", item.display()),

        Err(e) => println!("Error: {}", e),
    }
    

    if let Err(e) = inventory.remove_item("P1") {
        println!("Remove failed: {}", e)
    }
     if let Err(e) = inventory.remove_item("P1") {
        println!("Remove failed: {}", e)
    }

    println!(" \nCurrent Inventory :\n {}", inventory.display_all());
}
