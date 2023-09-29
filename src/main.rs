fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "-api" {
        println!("Hello, API!");
    } else {
        println!("Hello, CLI!");
    }
}
