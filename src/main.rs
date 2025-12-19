/********************************************************************
***  IHK Rust Developer 2025/2026                                 ***
***  m04_rust_teamwork_mm                                         ***
***  Topic : Mini-Datenbank mit Query-Engine                      ***
***  Fälligkeit 19.12.2025 23:59                                  ***
***  written by the authors Martin Hildebrand & Marc Grziwotz     ***
***  Projekt-Postfach  :   m04_rust:teamwork_mm@turbofisch.de     ***
***  2025 ©  ALL RIGHTS RESERVED                                  ***
*********************************************************************
***  Das Repository befindet sich hier:                           ***
***  https://github.com/martinscodingspace/m04_rust_teamwork_mm   ***
*********************************************************************
***  Diese Software steht unter folgender LIZENZ                  ***
***  GNU General Public License 3                                 ***
***  http://www.gnu.org/licenses/gpl-3.0.de.html                  ***
********************************************************************/


//!  Mini-Datenbank mit Query-Engine
//!
//!  1.1 Ziel des Projekts
//!  In diesem Projekt soll eine einfache, dateibasierte Datenbank mit einer kleinen
//!  Abfragesprache entwickelt werden.
//!
//!  1.2 Projektbeschreibung
//!  Die Aufgabe besteht darin, ein Datenbanksystem zu implementieren, das einfache
//!  Tabellen unterstützt, Daten persistent speichert und eine minimale Abfragesprache
//!  verarbeiten kann. Das fertige Programm soll ¨uber eine Kommandozeilenschnittstelle
//!  steuerbar sein und grundlegende Operationen wie INSERT, SELECT, DELETE und UPDATE
//!  ermöglichen.
//!
//!  1.3 Funktionale Anforderungen
//!
//!  1.3.1 Tabellen- und Datenmodell
//!  • Implementiert eine Struktur Table, die Datensätze in Form von Schlüssel-Wert-
//!    Paaren hält.
//!  • Unterst¨utzte Datentypen: String, i64, bool.
//!  • Jeder Datensatz soll als HashMap<String, Value> gespeichert werden, wobei Value
//!    ein eigenes Enum ist.
//!
//!  1.3.2 Persistente Speicherung
//!  • Entwickelt ein Modul storage, das Tabellen im JSON- oder Binärformat speichert.
//!  • Implementiert Funktionen zum Laden und Speichern einer gesamten Datenbankinstanz.
//!  • Fehler sollen über Result-Typen sauber kommuniziert werden.
//!
//!  1.3.3 Abfragesystem
//!
//!  • Implementiert eine einfache Abfragesprache mit folgender Syntax:
//!    INSERT INTO table_name { key1: value1, key2: value2 }
//!    SELECT field1, field2 FROM table_name WHERE key > 20
//!    DELETE FROM table_name WHERE key == true
//!    UPDATE table_name SET key = value WHERE id == 5
//!  • Entwickelt einen Parser, der diese Kommandos in Strukturen ¨uberführt.
//!  • Implementiert die Ausf¨uhrung der Befehle ¨uber ein eigenes Executor-Modul.
//!
//!  1.3.4 Kommandozeileninterface
//!
//!  • Das Programm soll als CLI-Tool nutzbar sein.
//!  • Es soll interaktiv sein (db>) oder Einmalbefehle annehmen.
//!  • Die Ergebnisse von SELECT-Abfragen sollen tabellarisch ausgegeben werden.
//!
//!  1.4 Erweiterungsm¨oglichkeiten (optional)
//!
//!  • Unterst¨utzung f¨ur mehrere Tabellen in einer Datenbank.
//!  • Konfigurierbares Speicherformat (Text/Bin¨ar).
//!  • Transaktionslog oder einfaches Journaling-System.
//!  • Benchmarks f¨ur Insert- und Select-Operationen.
//!  • Indexstrukturen
//!  – Implementiert für mindestens ein Feld pro Tabelle einen einfachen Index.
//!  – Der Index kann als Hash-Index oder als vereinfachter B-Baum dargestellt
//!    werden.
//!  – Der Index soll SELECT-Abfragen beschleunigen.

mod modules;

use crate::modules::executor;
use std::process::Command;

fn main() {
    clear_screen();
    println!("================================================================================");
    println!("=== Minidatenbank © 2025 Kundenverwaltungsprogramm inkl. Umsatz & Newsletter ===");
    println!("================================================================================");

    executor::start();

    println!();
    println!("================================================================================");
    println!("=== Programm - ENDE     Vielen Dank und bis zum nächsten Mal                 ===");
    println!("================================================================================");
    println!("\n");
}

pub fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("clear")
            .status()
            .unwrap();
    }
}
