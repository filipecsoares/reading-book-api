pub mod db {
    use crate::model::book::{Book, ReadingStatus};
    use std::error::Error;

    pub fn read_books(file_name: &str) -> Result<Vec<Book>, Box<dyn Error>> {
        let contents = std::fs::read_to_string(file_name)?;
        let books: Vec<Book> = serde_json::from_str(&contents)?;
        println!("Loaded {} books", books.len());
        Ok(books)
    }

    pub fn add_book(book: Book, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut books = read_books(file_name)?;
        books.push(book);
        std::fs::write(file_name, serde_json::to_string(&books)?)?;
        Ok(())
    }

    pub fn get_book(id: String, file_name: &str) -> Result<Book, Box<dyn Error>> {
        let books = read_books(file_name)?;
        Ok(books.into_iter().find(|b| b.id == id).unwrap())
    }

    pub fn delete_book(id: String, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut books = read_books(file_name)?;
        books.retain(|b| b.id != id);
        std::fs::write(file_name, serde_json::to_string(&books)?)?;
        Ok(())
    }

    pub fn update_book(book: Book, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut books = read_books(file_name)?;
        books.retain(|b| b.id != book.id);
        books.push(book);
        std::fs::write(file_name, serde_json::to_string(&books)?)?;
        Ok(())
    }

    pub fn complete_book(id: String, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut books = read_books(file_name)?;
        let book = books.iter_mut().find(|b| b.id == id).unwrap();
        book.reading_status = Some(ReadingStatus::Completed);
        std::fs::write(file_name, serde_json::to_string(&books)?)?;
        Ok(())
    }

    pub fn reading_book(id: String, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut books = read_books(file_name)?;
        let book = books.iter_mut().find(|b| b.id == id).unwrap();
        book.reading_status = Some(ReadingStatus::Reading);
        std::fs::write(file_name, serde_json::to_string(&books)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        db::db::{
            add_book, complete_book, delete_book, get_book, read_books, reading_book, update_book,
        },
        model::book::{Book, ReadingStatus},
    };

    fn create_test_file() -> &'static str {
        let file_name = format!("{}.json", uuid::Uuid::new_v4());
        std::fs::write(&file_name, "[]").unwrap();
        let file_name = Box::leak(file_name.into_boxed_str());
        file_name
    }

    #[test]
    fn test_read_books() {
        let file_name = create_test_file();
        let books = read_books(file_name).unwrap();
        assert_eq!(books.len(), 0);
        std::fs::remove_file(file_name).unwrap();
    }

    #[test]
    fn test_add_book() {
        let file_name = create_test_file();

        let book = Book::new(
            "title".to_string(),
            "author".to_string(),
            "isbn".to_string(),
            2000,
            100,
            Some(ReadingStatus::ToRead),
            None,
            None,
        );

        add_book(book.clone(), file_name).unwrap();

        let books = read_books(file_name).unwrap();
        assert_eq!(books.len(), 1);
        assert_eq!(books[0], book);
        std::fs::remove_file(file_name).unwrap();
    }

    #[test]
    fn test_get_book() {
        let file_name = create_test_file();

        let book = Book::new(
            "title".to_string(),
            "author".to_string(),
            "isbn".to_string(),
            2000,
            100,
            Some(ReadingStatus::ToRead),
            None,
            None,
        );

        add_book(book.clone(), file_name).unwrap();

        let added_book = get_book(book.id.clone(), file_name).unwrap();
        assert_eq!(book, added_book);
        std::fs::remove_file(file_name).unwrap();
    }

    #[test]
    fn test_delete_book() {
        let file_name = create_test_file();

        let book = Book::new(
            "title".to_string(),
            "author".to_string(),
            "isbn".to_string(),
            2000,
            100,
            Some(ReadingStatus::ToRead),
            None,
            None,
        );

        add_book(book.clone(), file_name).unwrap();

        delete_book(book.id, file_name).unwrap();

        let books = read_books(file_name).unwrap();
        assert_eq!(books.len(), 0);
        std::fs::remove_file(file_name).unwrap();
    }

    #[test]
    fn test_update_book() {
        let file_name = create_test_file();

        let book = Book::new(
            "title".to_string(),
            "author".to_string(),
            "isbn".to_string(),
            2000,
            100,
            Some(ReadingStatus::ToRead),
            None,
            None,
        );

        add_book(book.clone(), file_name).unwrap();

        let updated_book = Book::new(
            "The Rust Programming Language (2nd Edition)".to_string(),
            "Steve Klabnik, Carol Nichols, and Ryan Levick".to_string(),
            "isbn".to_string(),
            2021,
            200,
            Some(ReadingStatus::ToRead),
            None,
            None,
        );
        let updated_book = Book::new_with_id(book.id, updated_book);

        update_book(updated_book.clone(), file_name).unwrap();

        let books = read_books(file_name).unwrap();
        assert_eq!(books.len(), 1);
        assert_eq!(books[0], updated_book);
        std::fs::remove_file(file_name).unwrap();
    }

    #[test]
    fn test_complete_book() {
        let file_name = create_test_file();

        let book = Book::new(
            "title".to_string(),
            "author".to_string(),
            "isbn".to_string(),
            2000,
            100,
            Some(ReadingStatus::ToRead),
            None,
            None,
        );

        add_book(book.clone(), file_name).unwrap();

        complete_book(book.id, file_name).unwrap();

        let books = read_books(file_name).unwrap();
        assert_eq!(books.len(), 1);
        assert_eq!(books[0].reading_status, Some(ReadingStatus::Completed));
        std::fs::remove_file(file_name).unwrap();
    }

    #[test]
    fn test_reading_book() {
        let file_name = create_test_file();

        let book = Book::new(
            "title".to_string(),
            "author".to_string(),
            "isbn".to_string(),
            2000,
            100,
            Some(ReadingStatus::ToRead),
            None,
            None,
        );

        add_book(book.clone(), file_name).unwrap();

        reading_book(book.id, file_name).unwrap();

        let books = read_books(file_name).unwrap();
        assert_eq!(books.len(), 1);
        assert_eq!(books[0].reading_status, Some(ReadingStatus::Reading));
        std::fs::remove_file(file_name).unwrap();
    }
}
