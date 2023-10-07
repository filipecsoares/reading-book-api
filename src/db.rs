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

    pub fn add_book(book: Book) -> Result<(), Box<dyn Error>> {
        let mut books = read_books()?;
        books.push(book);
        std::fs::write(DB_FILENAME, serde_json::to_string(&books)?)?;
        Ok(())
    }

    pub fn get_book(id: String) -> Result<Book, Box<dyn Error>> {
        let books = read_books()?;
        Ok(books.into_iter().find(|b| b.id == id).unwrap())
    }

    pub fn delete_book(id: String) -> Result<(), Box<dyn Error>> {
        let mut books = read_books()?;
        books.retain(|b| b.id != id);
        std::fs::write(DB_FILENAME, serde_json::to_string(&books)?)?;
        Ok(())
    }

    pub fn update_book(book: Book) -> Result<(), Box<dyn Error>> {
        let mut books = read_books()?;
        books.retain(|b| b.id != book.id);
        books.push(book);
        std::fs::write(DB_FILENAME, serde_json::to_string(&books)?)?;
        Ok(())
    }
}