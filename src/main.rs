mod generator;
use generator::generate_log;
mod storage;
use storage::{write_log, read_all_logs};

//generates logs, writes them to file and then reads them back to verify
fn main() -> std::io::Result<()>{
    for i in 0..50{
        let entry = generate_log(i);
        println!("{} {} {} {}", entry.timestamp, entry.level, entry.source, entry.message);
        write_log("logs.txt", &entry)?;
    }
    
    let logs = read_all_logs("logs.txt")?;
    println!("File content:");
    for log in logs{
        println!("{}", log);
    }
    Ok(())
}
