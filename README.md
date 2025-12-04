MM-Teamwork presents:

# m04_rust_teamwork_mm

1 Projektaufgabe: Mini-Datenbank mit Query-Engine

1.1 Ziel des Projekts
In diesem Projekt soll eine einfache, dateibasierte Datenbank mit einer kleinen Abfragesprache
entwickelt werden.

1.2 Projektbeschreibung
Die Aufgabe besteht darin, ein Datenbanksystem zu implementieren, das einfache Tabellen
unterstuetzt, Daten persistent speichert und eine minimale Abfragesprache verarbeiten
kann. Das fertige Programm soll ueber eine Kommandozeilenschnittstelle steuerbar sein
und grundlegende Operationen wie INSERT, SELECT, DELETE und UPDATE erm¨oglichen.

1.3 Funktionale Anforderungen

1.3.1 Tabellen- und Datenmodell
• Implementiert eine Struktur Table, die Datens¨atze in Form von Schluessel-Wert-
Paaren haelt.
• Unterst¨utzte Datentypen: String, i64, bool.
• Jeder Datensatz soll als HashMap<String, Value> gespeichert werden, wobei Value
ein eigenes Enum ist.

1.3.2 Persistente Speicherung
• Entwickelt ein Modul storage, das Tabellen im JSON- oder Bin¨arformat speichert.
• Implementiert Funktionen zum Laden und Speichern einer gesamten Datenbankinstanz.
• Fehler sollen ¨uber Result-Typen sauber kommuniziert werden.

1.3.3 Abfragesystem
• Implementiert eine einfache Abfragesprache mit folgender Syntax:
INSERT INTO table_name { key1: value1, key2: value2 }
SELECT field1, field2 FROM table_name WHERE key > 20
DELETE FROM table_name WHERE key == true
UPDATE table_name SET key = value WHERE id == 5
• Entwickelt einen Parser, der diese Kommandos in Strukturen ueberfuehrt.
• Implementiert die Ausf¨uhrung der Befehle ¨uber ein eigenes Executor-Modul.

1.3.4 Kommandozeileninterface
• Das Programm soll als CLI-Tool nutzbar sein.
• Es soll interaktiv sein (db>) oder Einmalbefehle annehmen.
• Die Ergebnisse von SELECT-Abfragen sollen tabellarisch ausgegeben werden.

1.4 Erweiterungsmoeglichkeiten (optional)
• Unterstuetzung fuer mehrere Tabellen in einer Datenbank.
• Konfigurierbares Speicherformat (Text/Binaer).
• Transaktionslog oder einfaches Journaling-System.
• Benchmarks fuer Insert- und Select-Operationen.
• Indexstrukturen
– Implementiert fuer mindestens ein Feld pro Tabelle einen einfachen Index.
– Der Index kann als Hash-Index oder als vereinfachter B-Baum dargestellt werden.
– Der Index soll SELECT-Abfragen beschleunigen.
