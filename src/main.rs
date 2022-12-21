use std::{fs::File, io::BufReader};

mod parser;
mod executor;

fn main() {
    let mut sample = BufReader::new(File::open("sample_file.apo").unwrap());
    println!("{:#?}", parser::parse_file(&mut sample));
}
