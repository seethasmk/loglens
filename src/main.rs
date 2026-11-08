mod generator;
use generator::generate_log;
mod storage;
use storage::{write_log, read_all_logs};
mod index;
use index::{build_index, search};
use std::io;

fn main() -> std::io::Result<()> {
    // Generates fake logs and writes each one to the file
    for i in 0..20 {
        let entry = generate_log(i);
        println!("{} {} {} {}", entry.timestamp, entry.level, entry.source, entry.message);
        write_log("logs.txt", &entry)?;
    }

    // Read all logs back from the file for storage verification
    let logs = read_all_logs("logs.txt")?;
    println!("File content:");
    for log in &logs {
        println!("{}", log);
    }

    // Build a search index from logs, then repeatedly search until the user exits
    let word_index = build_index(&logs);
    loop {
        println!("Enter a keyword to search (or type exit to quit):");
        let mut keyword = String::new();
        io::stdin().read_line(&mut keyword)?;
        let keyword = keyword.trim();

        if keyword == "exit" {
            println!("Goodbye!");
            break;
        }

        let results = search(&word_index, keyword);
        if results.is_empty() {
            println!("No matches found for \"{}\"", keyword);
        } 
        else {
            println!("Found {} matches for \"{}\":", results.len(), keyword);
            for i in &results {
                println!("logs.txt:{}: {}", i, logs[*i]);
            }
        }
    }

    Ok(())
}