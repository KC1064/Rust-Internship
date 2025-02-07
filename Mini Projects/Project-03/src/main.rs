use std::time;
use rand::Rng;
mod tests;

enum BookStatus {
    // The book is available to be issued or reserved
    Available,

    // The book is currently issued to a user for a certain number of days at a certain timestamp
    Issued(u8, i32),

    // The book is reserved by a user
    Reserved,
}

struct Book {
    id: i32,
    author: i32,
    isbn: i32,
    status: BookStatus,
}

// DO NOT CHANGE
fn get_book_id() -> i32 { 
    let mut rng = rand::thread_rng(); 
    rng.gen_range(10000000..=99999999)
}

// DO NOT CHANGE
fn get_current_timestamp() -> i32 {
    let now = time::Instant::now();
    now.elapsed().as_secs() as i32
}

impl Book {
    // A new book is created with the status Available
    // The ID is generated randomly using the get_book_id function
    fn new(author: i32, isbn: i32) -> Book {
        // todo!();
        Book {
            id: get_book_id(),
            author,
            isbn,
            status: BookStatus::Available,
        }
    }

    // The book is issued to a user for a certain number of days
    // The status of the book is updated to Issued
    // Use the get_current_timestamp function to get the current timestamp
    fn issue_book(&self, no_of_days: u8) -> Book {
        // todo!();
        Book {
            id: self.id,
            author: self.author,
            isbn: self.isbn,
            status: BookStatus::Issued(no_of_days,get_current_timestamp()),
        }
    }

    // The book is reserved by a user
    // The status of the book is updated to Reserved
    fn reserve_book(&self) -> Book {
        Book {
            id: self.id,
            author: self.author,
            isbn: self.isbn,
            status: BookStatus::Reserved,
        }
    }

    // The book is returned by the user
    // The status of the book is updated to Available
    fn return_book(&self) -> Book {
        // todo!()
        Book {
            id: self.id,
            author: self.author,
            isbn: self.isbn,
            status: BookStatus::Available
        }
    }

    fn print_book_info(&self) {
        println!("-------------------------------------");
        println!("Book ID: {}", self.id);
        println!("Author: {}", self.author);
        println!("ISBN: {}", self.isbn);
        match &self.status {
            BookStatus::Available => println!("Status: Available"),
            BookStatus::Issued(days, timestamp) => println!("Status: Issued for {} days at timestamp {}", days, timestamp),
            BookStatus::Reserved => println!("Status: Reserved"),
        }     
    }
    
}

fn main() {
    let mut book = Book::new(1, 1234567890);

    book.print_book_info();

    book = book.issue_book(7);
    book.print_book_info();

    book = book.return_book();
    book.print_book_info();

    book = book.reserve_book();
    book.print_book_info();
    println!("-------------------------------------");

}







