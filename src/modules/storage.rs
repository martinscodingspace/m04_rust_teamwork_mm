use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::{fs, io};
use std::io::Write;
use colored::Colorize;
use crate::modules::graph_tools;
// use crate::modules::graph_tools::get_sep;

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
            println!("=== Neue Datenbank im JSON-Format wird auf diesem System erstellt            ===");
            println!("{}", graph_tools::get_sep());
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
    use std::io::{self};

    let mut name = String::new();
    let mut volume = String::new();
    let mut newsstring = String::new();

    println!();
    println!("{}", graph_tools::get_sep().green());
    println!("{}", "=== Willkommen im Unter-MENÜ  INSERT / Einfügen                              ===".green());
    println!("{}", graph_tools::get_sep().green());

    print!("=== EINGABE  Name                             : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("=== EINGABE  Umsatz                           : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut volume).unwrap();
    let volume: i32 = volume.trim().parse().unwrap();

    print!("=== EINGABE  Newsletter ( ja / nein )         : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut newsstring).unwrap();

    let newsletter = matches!(newsstring.trim().to_lowercase().as_str(), "ja" | "j" | "true");

    db.insert(
        name.trim().to_string(),
        EnCustomer::Status(CustomerValue {
            sales_volume: volume,
            newsletter,
        }),
    );

    println!("{}", graph_tools::get_sep().blue());
    println!("{}", "=== Eintrag wurde hinzugefügt                                                   ===".green());
    println!("{}", graph_tools::get_sep().blue());
}

pub fn select(db: &HashMap<String, EnCustomer>) {
    println!();
    println!("{}", graph_tools::get_sep().cyan());
    println!("{}", "=== Willkommen im Unter-MENÜ  SELECT / Suchen                                ===".cyan());
    println!("{}", graph_tools::get_sep().cyan());
    let mut name = String::new();
    print!("=== GESUCHTER Name                            : ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut name).unwrap();

    match db.get(name.trim()) {
        Some(c) => println!("=== Gefunden: {:?}", c),
        None => println!("{}", "=== Kein Eintrag gefunden                                                   ===".red()),
    }
}

pub fn delete(db: &mut HashMap<String, EnCustomer>) {
    println!("{}", graph_tools::get_sep().red());
    println!("{}", "=== Willkommen im Unter-MENÜ  DELETE / Löschen                               ===".red());
    println!("{}", graph_tools::get_sep().red());

    let mut name = String::new();
    print!("=== ZU LÖSCHENDER  Name                       : ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut name).unwrap();

    if db.remove(name.trim()).is_some() {
        println!("{}", "=== Eintrag gelöscht                                                         ===".green());
    } else {
        println!("{}", "=== Eintrag nicht gefunden                                                   ===".red());
    }
}

pub fn update(db: &mut HashMap<String, EnCustomer>) {
    println!("{}", graph_tools::get_sep().magenta());
    println!("{}", "=== Willkommen im Unter-MENÜ  UPDATE / Aktualisieren                         ===".magenta());
    println!("{}", graph_tools::get_sep().magenta());

    let mut name = String::new();
    let mut new_volume = String::new();
    let mut newsstring = String::new();

    print!("=== EINGABE  Name    (neu)                    : ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut name).unwrap();

    print!("=== EINGABE  Umsatz  (neu)                    : ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut new_volume).unwrap();
    let new_volume: i32 = new_volume.trim().parse().unwrap();

    print!("=== EINGABE  Newsletter aktuell ( ja / nein ) : ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut newsstring).unwrap();

    let newsletter = matches!(newsstring.trim().to_lowercase().as_str(), "ja" | "j" | "true");

    if let Some(EnCustomer::Status(cust)) = db.get_mut(name.trim()) {
        cust.sales_volume = new_volume;
        cust.newsletter = newsletter;
        println!("{}", "=== Eintrag wurde aktualsiert                                                ===".green());
    } else {
        println!("{}", "=== Eintrag nicht gefunden                                                   ===".red());
    }
}

