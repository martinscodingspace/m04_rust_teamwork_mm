
use crate::storage;
use std::io::{self, Write};

use serde::{Serialize, Deserialize};
use crate::storage::load;

#[derive(Serialize, Deserialize)]
struct CustomerValue {
    sales_volume: i32,
    newsletter: bool,
}

#[derive(Serialize, Deserialize)]
enum EnCustomer {
    Status(CustomerValue),
}

pub fn start() {
    let mut map = load();

    loop {
        // Das fertige Programm soll ¨uber eine Kommandozeilenschnittstelle steuerbar sein
        // und grundlegende Operationen wie INSERT, SELECT, DELETE und UPDATE ermöglichen
        let mut eingabe = String::new();
        println!("\nDas Auswahl-Menü\n");
        println!("Einfügen       / INSERT   < i >");
        println!("Suchen         / SELECT   < s >");
        println!("Löschen        / DELETE   < d >");
        println!("Aktualisieren  / UPDATE   < u >");
        println!("Beenden        / END      < x >");
        print!("Bitte Auswahl treffen: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut eingabe).unwrap();

        match eingabe.trim() {
            "i" | "I" => storage::insert(&mut map),
            "s" | "S" => storage::select(&map),
            "d" | "D" => storage::delete(&mut map),
            "u" | "U" => storage::update(&mut map),
            "x" | "X" => {
                storage::save(&map);
                break;
            },
            _ => continue,
        }
    }
}

