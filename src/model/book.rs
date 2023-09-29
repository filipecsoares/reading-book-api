use chrono::{NaiveDate, Utc};

#[derive(Debug)]
pub struct Book {
    id: i32,
    title: String,
    author: String,
    isbn: String,
    year: i16,
    pages: i16,
    reading_status: ReadingStatus,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

#[derive(Debug, PartialEq)]
pub enum ReadingStatus {
    ToRead,
    Reading,
    Completed,
}