use crate::modules::graph_tools;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use colored::Colorize;


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnValue {
    Werte { sales_volume: i64, newsletter: bool },
}

const FILE_PATH: &str = "mm_database.json";

pub fn insert_json(nom:String, vol: i64, new: bool) -> std::io::Result<()> {
    let mut map: HashMap<String, EnValue> = HashMap::new();
    map.insert(
        nom.to_string(),
        EnValue::Werte { sales_volume: vol, newsletter: new },
    );


    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)   // Leert die Datei vor dem Schreiben, falls gewünscht
        .open(FILE_PATH)?;

    let _ =serde_json::to_writer_pretty(file, &map)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));

    println!("HashMap wurde erfolgreich nach  mm_database.json  geschrieben.");

    Ok(()) 
}

pub fn update_json() -> std::io::Result<()> {
    
    let mut map = load_from_json_file(FILE_PATH)?;
    let mut file = File::open(FILE_PATH)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialisieren
    let map: HashMap<String, EnValue> = serde_json::from_str(&contents).expect("Failed to parse JSON");

    println!("\nNach dem Update geladene Daten: {:?}", map);
    Ok(())
}

pub fn load_from_json_file(path: &str) -> std::io::Result<HashMap<String, EnValue>> {
    
    if !Path::new(path).exists() || std::fs::metadata(path)?.len() == 0 {
        return Ok(HashMap::new());
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialisieren
    serde_json::from_str(&contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn insert_data() {
    let mut inp = String::new();
    let mut nom = String::new();
    let mut vol = 0i64;
    let mut new = false;
    loop {
        inp.clear();
        println!();
        println!("{}", graph_tools::get_sep().magenta());
        println!("{}", graph_tools::get_umenu_insert().magenta());
        println!("{}", graph_tools::get_sep().magenta());
        print!("=== Name des Kunden   < x = zurück >   :        ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inp).unwrap();
        nom = inp.trim().to_string();
        if nom == "x" || nom == "X" { break; }
        inp.clear();
        print!("=== Umsatz-Volumen                     :        ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inp).unwrap();
        let vol = match inp.trim().parse::<i64>() {
            Ok(vol) => vol,
            Err(_) => {
                println!();
                println!("{}", graph_tools::get_sep().red());
                println!("{}", graph_tools::get_error_liner().red());
                println!("{}", graph_tools::get_sep().red());
                continue;
            }
        };
        inp.clear();
        print!("=== Newsletter vorhanden   ( j / n )   :        ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inp).unwrap();
        new = if inp.to_string() == "j" || inp.to_string() == "J" {true} else {false};
    }
    let _ret = insert_json(nom, vol, new);
}

pub fn select_data() {
    let mut inp = String::new();
    let mut nom = String::new();
    println!();
    println!("{}", graph_tools::get_sep().cyan());
    println!("{}", graph_tools::get_umenu_select().cyan());
    println!("{}", graph_tools::get_sep().cyan());
    print!("=== Name des Kunden   < x = zurück >   :        ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut inp).unwrap();
    nom = inp.trim().to_string();
}

pub fn delete_data() {
    let mut inp = String::new();
    let mut nom = String::new();
    println!();
    println!("{}", graph_tools::get_sep().red());
    println!("{}", graph_tools::get_umenu_delete().red());
    println!("{}", graph_tools::get_sep().red());
    print!("=== Name des Kunden   < x = zurück >   :        ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut inp).unwrap();
    nom = inp.trim().to_string();
}

pub fn update_data() {
    println!();
    println!("{}", graph_tools::get_sep().magenta());
    println!("{}", graph_tools::get_umenu_update().magenta());
    println!("{}", graph_tools::get_sep().magenta());
    let _ret = update_json();

}
