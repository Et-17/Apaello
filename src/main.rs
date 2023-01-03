use std::{fs::File, io::BufReader};

mod executor;
mod parser;

fn main() {
    let mut sample = BufReader::new(File::open("sample_file.apo").unwrap());
    let line = &parser::parse_file(&mut sample).unwrap().lines[0].words[..];
    println!("{:#?}", executor::packing::pack_words(line));
}
