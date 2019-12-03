#[macro_export]
macro_rules! lines_from {
    ($filename:expr) => {
        {
            use std::io::BufRead;
            let file = std::fs::File::open(format!("resources/{}.txt", $filename)).unwrap();
            let reader = std::io::BufReader::new(file);
            reader.lines()
        }
    }
}

#[macro_export]
macro_rules! csv_from {
    ($filename:expr) => {
        std::fs::read_to_string(format!("resources/{}.txt", $filename)).unwrap().split(",")
    }
}
