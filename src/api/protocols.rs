use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::model::book::ReadingStatus;

#[derive(Debug, Deserialize, Serialize)]
pub struct BookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub year: i16,
    pub pages: i16,
    pub reading_status: Option<ReadingStatus>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}
