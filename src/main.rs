use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = minigrep::run(&args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
}