pub trait Communicate {
    fn get_line(&self) -> String;
    fn get_char(&self) -> Option<char>;
    fn say(&self, message: &str);
    fn say_line(&self, message: &str);
}

use std::cell::RefCell;
use std::io::{self, Write};

pub struct IoCommunicate {
    stdin: io::Stdin,
    stdout: RefCell<io::Stdout>,
}

impl IoCommunicate {
    pub fn new() -> IoCommunicate {
        IoCommunicate {
            stdin: std::io::stdin(),
            stdout: RefCell::new(std::io::stdout()),
        }
    }

    fn flush(&self) {
        if let Ok(mut stdout) = self.stdout.try_borrow_mut() {
            stdout.flush().expect("Unable to flush stdout.");
        }
    }
}

impl Communicate for IoCommunicate {
    fn get_line(&self) -> String {
        let mut line = String::new();
        self.stdin
            .read_line(&mut line)
            .expect("Unable to read from stdin.");
        line
    }

    fn get_char(&self) -> Option<char> {
        self.get_line().chars().next()
    }

    fn say(&self, message: &str) {
        print!("{}", message);
        self.flush();
    }

    fn say_line(&self, message: &str) {
        println!("{}", message);
        self.flush();
    }
}
