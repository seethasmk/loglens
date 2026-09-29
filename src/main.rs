mod generator;
use generator::generate_log;

fn main() {
    for i in 0..12{
        let entry = generate_log(i);
        println!("{} {} {} {}", entry.timestamp, entry.level, entry.source, entry.message);
    }
}
