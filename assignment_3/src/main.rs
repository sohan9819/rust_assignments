use std::{
    fmt::{Display as FmtDisplay, Formatter, Result},
    fs::File,
    io::{Read, Write},
    path::{self, Display, Path},
};

#[derive(Debug)]
enum LogType {
    INFO,
    ERROR,
}

impl FmtDisplay for LogType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LogType::INFO => write!(f, "INFO"),
            LogType::ERROR => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&mut self, log_type: LogType, msg: &str);
}

struct ConsoleLogger;
impl ConsoleLogger {
    fn new() -> Self {
        Self
    }
}

impl Logger for ConsoleLogger {
    fn log(&mut self, log_type: LogType, msg: &str) {
        println!("[{:?}]:{}", log_type, msg);
    }
}

struct FileLogger {
    file_name: &'static str,
    file: File,
    display: Display<'static>,
    path: &'static Path,
}
impl FileLogger {
    fn new(file_name: &'static str) -> Self {
        let path = Path::new(file_name);
        let display = path.display();

        // Create a file
        let file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        FileLogger {
            file_name,
            file,
            display,
            path,
        }
    }

    fn read_logs(&mut self) {
        // Read the file contents into a string, returns `io::Result<usize>`
        let mut file = match File::open(&self.path) {
            Err(why) => panic!("couldn't open {}: {}", self.display, why),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", self.display, why),
            Ok(_) => println!("{} contains:\n{}", self.display, s),
        }
    }
}

impl Logger for FileLogger {
    fn log(&mut self, log_type: LogType, msg: &str) {
        match self
            .file
            .write_all(format!("[{}] {}\n", log_type, msg).as_bytes())
        {
            Err(why) => panic!("couldn't write to {}: {}", self.display, why),
            Ok(_) => println!("successfully wrote to {}", self.display),
        }
    }
}

fn main() {
    println!("Hello, world!");

    // let path = Path::new("data.txt");
    // let display = path.display();

    // // Create a file
    // let mut file = match File::create(&path) {
    //     Err(why) => panic!("couldn't create {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // Write a file
    // match file.write_all("[App Started] Hello world!!!\n".as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }
    // match file.write_all("[App Started] Hello world!!!\n".as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }

    // // Open a file
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // Read the file contents into a string, returns `io::Result<usize>`
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => println!("{} contains:\n{}", display, s),
    // }

    let mut file_logger = FileLogger::new("logger.txt");
    let mut console_logger = ConsoleLogger::new();

    console_logger.log(LogType::INFO, "app started");
    file_logger.log(LogType::INFO, "app started");
    console_logger.log(LogType::ERROR, "Hello world!!!");
    file_logger.log(LogType::ERROR, "Hello world!!!");

    file_logger.read_logs();
}
