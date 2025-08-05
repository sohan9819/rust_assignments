trait Logger {
    fn error();
    fn info();
}

struct ConsoleLogger;
impl ConsoleLogger {
    fn new() -> Self {
        ConsoleLogger {}
    }
}

struct FileLogger;
impl FileLogger {
    fn new() -> Self {
        FileLogger {}
    }
}

fn main() {
    println!("Hello, world!");
}
