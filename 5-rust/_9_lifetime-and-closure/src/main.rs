use core::fmt;
use std::collections::{hash_map::Entry, HashMap};

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
                write!(f, "Item with id '{}' already exists", id)
            }
            InventoryError::InvalidId => write!(f, "Invalid id"),
            InventoryError::ItemNotFound(id) => {
                write!(f, "Item with id '{}' not found", id)
            }
        }
    }
}

struct Inventory<'a, T>
where
    T: DisplayItem,
{
    items: HashMap<String, &'a T>,
}

impl<'a, T> Inventory<'a, T>
where
    T: DisplayItem,
{
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, id: impl Into<String>, item: &'a T) -> Result<(), InventoryError> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }

        match self.items.entry(id.clone()) {
            Entry::Occupied(_) => Err(InventoryError::DuplicateId(id)),
            Entry::Vacant(e) => {
                e.insert(item);
                Ok(())
            }
        }
    }

    fn get_item(&self, id: &str) -> Result<&'a T, InventoryError> {
        self.items
            .get(id)
            .copied()
            .ok_or_else(|| InventoryError::ItemNotFound(id.to_string()))
    }

    fn remove_item(&mut self, id: &str) -> Result<&'a T, InventoryError> {
        self.items
            .remove(id)
            .ok_or_else(|| InventoryError::ItemNotFound(id.to_string()))
    }

    fn display_all_with<F>(&self, formatter: F) -> String
    where
        F: Fn(&str, &T) -> String,
    {
        if self.items.is_empty() {
            return "Inventory is empty".to_string();
        }

        self.items
            .iter()
            .map(|(id, item)| formatter(id, item))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

struct Product {
    name: String,
    price: f64,
}

impl DisplayItem for Product {
    fn display(&self) -> String {
        format!("Product: {}, Price: ₹{}", self.name, self.price)
    }
}

fn main() {
    let laptop = Product {
        name: "Laptop".into(),
        price: 60000.0,
    };

    let neckband = Product {
        name: "Neckband".into(),
        price: 1500.0,
    };

    let mut inventory = Inventory::<Product>::new();
    println!(
        "\nInventory:\n{}",
        inventory.display_all_with(|id, item| { format!("ID: {}\n{}\n", id, item.display()) })
    );

    inventory.add_item("P1", &laptop).unwrap();
    inventory.add_item("P2", &neckband).unwrap();

    println!(
        "\nInventory:\n{}",
        inventory.display_all_with(|id, item| { format!("ID: {}\n{}\n", id, item.display()) })
    );
    match inventory.get_item("P2") {
        Ok(item) => println!("Fetched: {}", item.display()),
        Err(e) => println!("Error: {}", e),
    }

    match inventory.remove_item("P1") {
        Ok(item) => println!("Removed: {}", item.display()),
        Err(e) => println!("Remove error: {}", e),
    }

    println!(
        "\nInventory:\n{}",
        inventory.display_all_with(|id, item| { format!("ID: {}\n{}\n", id, item.display()) })
    );
}
