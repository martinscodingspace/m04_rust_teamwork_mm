mod storage;
use crate::executor::storage::*;
use std::collections::HashMap;
use std::io::{self, Write};

struct CustomerValue {
    sales_volume: i32,
    newsletter  : bool
}

enum EnCustomer {
    Status(CustomerValue)
}

pub fn start() {
    let mut customer_map : HashMap<String, EnCustomer> = HashMap::new();
    let mut maps_list: Vec<HashMap<String, EnCustomer>> = Vec::new();

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
        print!  ("Bitte Auswahl treffen     : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut eingabe).unwrap();
        match eingabe.trim() {
            "i" | "I" => storage::insert(),
            "s" | "S" => storage::select(),
            "d" | "D" => storage::delete(),
            "u" | "U" => storage::update(),
            "x" | "X" => { break },
            _ => continue,
        }

    }
}

