struct Book {
    title: String,
    author: String,
    checked_out: bool,
}

// Add a book (takes ownership)
fn add_book(library: &mut Vec<Book>, book: Book) {
    library.push(book);
}

// List books (immutable borrow)
fn list_books(library: &Vec<Book>) {
    println!("\n📚 Library Books:");
    for book in library {
        println!(
            "- '{}' by {} [{}]",
            book.title,
            book.author,
            if book.checked_out { "Checked Out" } else { "Available" }
        );
    }
}

// Check out a book by title (mutable borrow)
fn checkout_book(library: &mut Vec<Book>, title: &str) {
    for book in library.iter_mut() {
        if book.title == title {
            if book.checked_out {
                println!("⚠ '{}' is already checked out!", title);
            } else {
                book.checked_out = true;
                println!("✅ You checked out '{}'", title);
            }
            return;
        }
    }

    println!("❌ Book '{}' not found in library", title);
}

fn main() {
    let mut library: Vec<Book> = Vec::new();

    // Add books
    add_book(&mut library, Book {
        title: "The Hobbit".to_string(),
        author: "J.R.R. Tolkien".to_string(),
        checked_out: false,
    });

    add_book(&mut library, Book {
        title: "1984".to_string(),
        author: "George Orwell".to_string(),
        checked_out: false,
    });

    add_book(&mut library, Book {
        title: "To Kill a Mockingbird".to_string(),
        author: "Harper Lee".to_string(),
        checked_out: false,
    });

    // List books
    list_books(&library);

    // Checkout a book
    checkout_book(&mut library, "1984");

    // List again to show change
    list_books(&library);
}