use crate::generator::LogEntry;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::DateTime;

//writes one log entry to the file
pub fn write_log(path: &str, entry: &LogEntry) -> std::io::Result<()>{
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    let datetime = DateTime::from_timestamp(entry.timestamp as i64, 0).unwrap();
    writeln!(file, "{} {} {} {}", datetime, entry.level, entry.source, entry.message)?;
    Ok(())
}

//reads all log lines from a file
pub fn read_all_logs(path: &str) -> std::io::Result<Vec<String>>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut logs = Vec::new();
    for line in reader.lines(){
        logs.push(line?);
    }
    Ok(logs)
}