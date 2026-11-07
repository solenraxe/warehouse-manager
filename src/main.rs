#![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::HashMap;

#[derive(Debug)]
enum Category {
    Electronics,
    Clothes,
    Tools,
    Misc,
}

#[derive(Debug)]
struct Item {
    name: String,
    quantity: u32,
    category: Category,
}

#[derive(Debug)]
struct Warehouse {
    items: Vec<Item>,
    stock: HashMap<String, u32>,
}

impl Warehouse {
    fn new() -> Warehouse {
        Warehouse {
            items: Vec::new(),
            stock: HashMap::new(),
        }
    }

    fn addItem(&mut self, name: String, qty: Option<u32>, cat: Option<Category>) {
        let q = qty.unwrap_or(1);
        if let Some(existing) = self.stock.get_mut(&name) {
            *existing += q;
            for item in &mut self.items {
                if item.name == name {item.quantity += q;}
            }
        } else {
            let item = Item {name: name.clone(), quantity: q, category: cat.unwrap_or(Category::Misc)};
            self.items.push(item);
            self.stock.insert(name, q);
        }
    }

    fn removeItem(&mut self, name: String, qty: Option<u32>) {
        let q = qty.unwrap_or(1);
        if let Some(existing) = self.stock.get_mut(&name) {
            *existing -= q;
            if *existing <= 0 {self.stock.remove(&name);}
        }
        if let Some(index) = self.items.iter().position(|i| *i.name == name) {
            self.items[index].quantity -= q;
            if self.items[index].quantity <= 0 {self.items.swap_remove(index);}
        }
    }
}

fn main() {
    let mut warehouse = Warehouse::new();
    warehouse.addItem(String::from("Screwdriver"), Some(5), Some(Category::Tools));
    warehouse.addItem(String::from("Screwdriver"), Some(1), Some(Category::Tools));
    warehouse.removeItem(String::from("Screwdriver"), Some(2));
    println!("{:#?}", warehouse);
}