mod api;
use api::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "-api" {
        routes::run();
    } else {
        println!("Hello, CLI!");
    }
}
