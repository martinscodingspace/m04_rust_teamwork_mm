// mod storage;
use crate::modules::storage;
use crate::modules::storage::insert_data;
use crate::modules::storage::select_data;
use crate::modules::storage::delete_data;
use crate::modules::storage::update_data;
use colored::Colorize;
use crate::modules::graph_tools;
use std::io::{self, Write};
pub fn start() {
    let mut eingabe = String::new();
    let mut auswahl;
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
        print!("=== Bitte Ihre Auswahl                 :        ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut eingabe).unwrap();
        eingabe = eingabe.trim().to_lowercase();
        auswahl = eingabe.to_string();
        println!("{}", graph_tools::get_sep().blue());
        if eingabe == "x" || eingabe == "X" {
            break;
        }
        if eingabe != "i" && eingabe != "s" && eingabe != "d" && eingabe != "u" {
            println!();
            println!("{}", graph_tools::get_sep().red());
            println!("{}", graph_tools::get_error_liner().red());
            println!("{}", graph_tools::get_sep().red());
            continue;
        }
        if auswahl == "i" { let _ = storage::insert_data(); };
        if auswahl == "s" { storage::select_data(); };
        if auswahl == "d" { storage::delete_data(); };
        if auswahl == "u" { storage::update_data(); };
    }

}

