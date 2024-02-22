use crate::db::db;

const DB_FILENAME: &str = ".books.json";

pub fn run() {
    println!("Managing Books, CLI!");
    print_menu();
    println!("Please enter your option:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "2" => view_books(),
        _ => println!("Invalid option"),
    }
    println!("You entered: {}", input);
}

fn view_books() {
    let books = db::read_books(DB_FILENAME).expect("Failed to read books");
    println!("Books: {:?}", books);
}

fn print_menu() {
    println!("1. Add a book");
    println!("2. View all books");
    println!("3. Remove a book");
    println!("4. Update a book");
    println!("5. Exit");
}

#[cfg(test)]
mod tests {
    use crate::db::db;

    fn create_test_file() -> &'static str {
        let file_name = format!("{}.json", uuid::Uuid::new_v4());
        std::fs::write(&file_name, "[]").unwrap();
        let file_name = Box::leak(file_name.into_boxed_str());
        file_name
    }

    #[test]
    fn test_view_books() {
        let file_name = create_test_file();
        let books = db::read_books(file_name).expect("Failed to read books");
        assert_eq!(books.len(), 0);
        std::fs::remove_file(file_name).unwrap();
    }
}
