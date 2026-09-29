use std::time::{SystemTime, UNIX_EPOCH};

//single log entry in the system
pub struct LogEntry{
    pub timestamp: u64,    //seconds since Unix Epoch
    pub level: String,     //severity levels
    pub source: String,    //which server generated this log
    pub message: String,   //log message
}

//generates log entry based on the sequence no
pub fn generate_log(seq: u64) -> LogEntry{
    let sources = ["server-1", "server-2", "server-3"];

    //tuple pairs of severity level and the log message
    let log_templates = [
        ("INFO", "user logged in"),
        ("INFO", "payment processed"),
        ("INFO", "health check passed"),
        ("WARN", "login failed"),
        ("WARN", "cache miss"),
        ("WARN", "disk usage high"),
        ("ERROR", "payment timeout"),
        ("ERROR", "request timeout"),
        ("ERROR", "connection reset"),
        ("CRITICAL", "database unreachable"),
        ("CRITICAL", "service crashed"),
        ("CRITICAL", "out of memory"),
    ];

    let source = sources[(seq as usize) % sources.len()];
    let (level, message) = log_templates[(seq as usize) % log_templates.len()];
    let timestamp = SystemTime:: now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    LogEntry{
        timestamp,
        level: level.to_string(),
        source: source.to_string(),
        message: message.to_string(),
    }
}