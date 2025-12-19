use crate::modules::storage;
use crate::modules::graph_tools;
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, Write};

pub fn start() {
    let mut db: HashMap<String, storage::EnCustomer> = storage::load();

    let mut eingabe = String::new();

    loop {
        eingabe.clear();

        println!();
        println!("{}", graph_tools::get_sep().blue());
        println!("{}", "=== Willkommen im Haupt-MENÜ           Bitte folgende Auswahl treffen:       ===".blue());
        println!("{}", graph_tools::get_sep().blue());
        println!("{}", "=== INSERT / Datensatz Einfügen        PRESS  < i >                          ===".blue());
        println!("{}", "=== SELECT / Datensatz Suchen          PRESS  < s >                          ===".blue());
        println!("{}", "=== DELETE / Datensatz Löschen         PRESS  < d >                          ===".blue());
        println!("{}", "=== UPDATE / Datensatz Aktualisieren   PRESS  < u >                          ===".blue());
        println!("{}", "=== END    / Programm  BEENDEN         PRESS  < x >                          ===".yellow());
        println!("{}", graph_tools::get_sep().blue());

        print!("=== Bitte Ihre Auswahl                        : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut eingabe).unwrap();
        let auswahl = eingabe.trim().to_lowercase();

        match auswahl.as_str() {
            "i" => {
                storage::insert(&mut db);
                storage::save(&db);
            }
            "s" => storage::select(&db),
            "d" => {
                storage::delete(&mut db);
                storage::save(&db);
            }
            "u" => {
                storage::update(&mut db);
                storage::save(&db);
            }
            "x" => break,
            _ => {
                println!("{}", graph_tools::get_error_liner().red());
            }
        }
    }
}
