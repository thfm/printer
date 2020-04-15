use colored::Colorize;
use std::{
    io::{self, Write},
    path::PathBuf,
    thread, time,
};

struct Printer {
    queue: Vec<Document>,
}

impl Printer {
    // Initialises the printer with an empty queue
    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    fn add_to_queue(&mut self, document: Document) {
        self.queue.push(document);
    }

    // 'Prints' the first document in the queue, if there is one
    fn print(&mut self) {
        // Checks that the queue is not empty
        if self.queue.len() > 0 {
            // Retrieves and removes the first document
            let document = self.queue.remove(0);
            println!(
                "{}",
                format!("Printing '{}'\n", document.path.display())
                    .green()
                    .bold()
            );
            for (i, page) in document.pages.iter().enumerate() {
                // Displays the current page number
                println!("{} Page {}:", "->".bright_black(), i);
                // Prints each character of the page, one by one
                for c in page.chars() {
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                    thread::sleep(time::Duration::from_millis(50));
                }
                println!("\n");
            }
            println!("{}\n", "Finished".bright_blue());
        }
    }
}

struct Document {
    path: PathBuf,
    pages: Vec<String>,
}

fn main() {
    // Initialises a printer instance that allows for multiple owners (with Arc)
    // and protects against simultaneous mutations (with Mutex)
    let p = std::sync::Arc::new(std::sync::Mutex::new(Printer::new()));

    let mut threads = Vec::with_capacity(2);

    // Adds the same document to the printer queue every 5 seconds
    {
        let p = p.clone();
        threads.push(std::thread::spawn(move || loop {
            p.lock().unwrap().add_to_queue(Document {
                path: PathBuf::from("the_best_document.java"),
                pages: vec![
                    "System.out.println('Hello, World!');".to_string(),
                    "println!(\"Hello, World!\");".to_string(),
                    "print('Hello')".to_string(),
                ],
            });
            thread::sleep(time::Duration::from_secs(5));
        }));
    }

    // Constantly prints the documents in the printer queue
    {
        let p = p.clone();
        threads.push(std::thread::spawn(move || loop {
            p.lock().unwrap().print();
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
