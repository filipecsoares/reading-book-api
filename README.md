# Book Library Management with Reading Progress Tracker

## Project Overview

The "Book Library Management with Reading Progress Tracker" is a Rust-based study project focused on building a robust API to manage a personal book collection. The project integrates features to track reading progress, including book reading status, start date, and end date. Utilizing enums, structs, error handling, and the chrono crate for date management, this project offers hands-on practice in designing and implementing a functional and efficient Rust API. It allows users to organize their books, record reading progress, and seamlessly update book information as they progress through their reading journey.

## Key Features

1. Book Struct:
    - Define a struct representing essential book details such as title, author, ISBN, published year, reading status, start date, and end date of reading.
2. Reading Status Tracking:
    - Implement an enum to track the reading status of each book, including "To Read", "Reading", and "Read" states.
3. Date Management:
    - Utilize the chrono crate to handle dates, enabling the tracking of the start and end dates for each book's reading session.
4. CRUD Operations:
    - Implement Create, Read, Update, and Delete (CRUD) operations for books, allowing users to manage their book collection and update reading progress.
5. Error Handling:
    - Introduce error handling mechanisms and custom error types to provide informative feedback in case of invalid inputs or operations.
6. CLI Interface:
    - Develop a simple command-line interface to interact with the API, enabling users to perform actions like adding, viewing, updating, and deleting books in their collection.
7. Web API:
    - Develop a simple web API using actix to manage books by sending requests to the API.
