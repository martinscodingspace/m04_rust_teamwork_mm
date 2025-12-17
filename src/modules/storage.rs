use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerValue {
    pub sales_volume: i32,
    pub newsletter: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnCustomer {
    Status(CustomerValue),
}

const FILE_NAME: &str = "mm_database.json";

pub fn load() -> HashMap<String, EnCustomer> {
    match fs::read_to_string(FILE_NAME) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| {
            println!("JSON beschädigt, neue Datenbank wird erstellt.");
            HashMap::new()
        }),
        Err(_) => {
            println!("Datei nicht gefunden, neue Datenbank wird erstellt.");
            HashMap::new()
        }
    }
}

pub fn save(db: &HashMap<String, EnCustomer>) {
    let json = serde_json::to_string_pretty(db).unwrap();
    fs::write(FILE_NAME, json).unwrap();
}

pub fn insert(db: &mut HashMap<String, EnCustomer>) {
    use std::io::{self, Read};

    let mut name = String::new();
    let mut volume = String::new();
    let mut newsstring = String::new();

    println!("Name eingeben:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Umsatz eingeben:");
    io::stdin().read_line(&mut volume).unwrap();
    let volume: i32 = volume.trim().parse().unwrap();

    println!("Newsletterstatus eingeben (ja/nein):");
    io::stdin().read_line(&mut newsstring).unwrap();

    let newsletter = matches!(newsstring.trim().to_lowercase().as_str(), "ja" | "j" | "true");

    db.insert(
        name.trim().to_string(),
        EnCustomer::Status(CustomerValue {
            sales_volume: volume,
            newsletter,
        }),
    );

    println!("Eintrag hinzugefügt.");
}

pub fn select(db: &HashMap<String, EnCustomer>) {
    let mut name = String::new();
    println!("Name suchen:");
    std::io::stdin().read_line(&mut name).unwrap();

    match db.get(name.trim()) {
        Some(c) => println!("Gefunden: {:?}", c),
        None => println!("Kein Eintrag gefunden!"),
    }
}

pub fn delete(db: &mut HashMap<String, EnCustomer>) {
    let mut name = String::new();
    println!("Name löschen:");
    std::io::stdin().read_line(&mut name).unwrap();

    if db.remove(name.trim()).is_some() {
        println!("Eintrag gelöscht.");
    } else {
        println!("Eintrag nicht gefunden.");
    }
}

pub fn update(db: &mut HashMap<String, EnCustomer>) {
    let mut name = String::new();
    let mut new_volume = String::new();
    let mut newsstring = String::new();

    println!("Name eingeben:");
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Neuer Umsatz:");
    std::io::stdin().read_line(&mut new_volume).unwrap();
    let new_volume: i32 = new_volume.trim().parse().unwrap();

    println!("Newsletterstatus eingeben (ja/nein):");
    std::io::stdin().read_line(&mut newsstring).unwrap();

    let newsletter = matches!(newsstring.trim().to_lowercase().as_str(), "ja" | "j" | "true");

    if let Some(EnCustomer::Status(cust)) = db.get_mut(name.trim()) {
        cust.sales_volume = new_volume;
        cust.newsletter = newsletter;
        println!("Eintrag aktualisiert.");
    } else {
        println!("Eintrag nicht gefunden.");
    }
}

