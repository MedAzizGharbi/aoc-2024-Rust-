mod days;
// mod utils;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day>");
        return;
    }
    let day = &args[1];
    match day.as_str() {
        "1" => days::day01::run(),
        "2" => days::day02::run(),
        "3" => days::day03::run(),
        "4" => days::day04::run(),
        _ => println!("Pick a valid date!"),
    }
}
