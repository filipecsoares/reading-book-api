pub mod db {
    use std::error::Error;
    use crate::model::book::Book;

    const DB_FILENAME: &str = ".books.json";

    pub fn read_books() -> Result<Vec<Book>, Box<dyn Error>> {
        let contents = std::fs::read_to_string(DB_FILENAME)?;
        let books: Vec<Book> = serde_json::from_str(&contents)?;
        println!("Loaded {} books", books.len());
        Ok(books)
    }
}