use chrono::{NaiveDate, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    author: String,
    isbn: String,
    year: i16,
    pages: i16,
    pub reading_status: ReadingStatus,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl Book {
    pub fn new(
        title: String,
        author: String,
        isbn: String,
        year: i16,
        pages: i16,
        reading_status: ReadingStatus,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Self {
        let id = Uuid::new_v4().hyphenated().to_string();
        Self {
            id,
            title,
            author,
            isbn,
            year,
            pages,
            reading_status,
            start_date,
            end_date,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ReadingStatus {
    ToRead,
    Reading,
    Completed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let book = Book::new(
            "title".to_string(),
            "author".to_string(),
            "isbn".to_string(),
            2000,
            100,
            ReadingStatus::ToRead,
            None,
            None,
        );
        assert!(book.id.len() > 0);
        assert_eq!(book.reading_status, ReadingStatus::ToRead);
        assert_eq!(book.title, "title");
    }
}
