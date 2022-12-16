pub mod structure;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_file(file: &mut BufReader<File>) -> Result<structure::File, String> {
    let file_lines = file.lines().enumerate();
    let mut lines: Vec<structure::Line> = Vec::new();
    for (num, line) in file_lines {
        let unwrapped = line.unwrap();
        let trimmed = unwrapped.trim();

        if trimmed.is_empty() {
            continue;
        }

        lines.push(parse_line(trimmed, num + 1)?);
    }
    Ok(structure::File { lines })
}

pub fn parse_line(line: &str, number: usize) -> Result<structure::Line, String> {
    let mut words = Vec::new();
    let split = line.split_whitespace();
    let mut in_comment = false;
    for word in split {
        match word {
            "\\\\" => break,
            "(" => in_comment = true,
            ")" => in_comment = false,
            _ => {
                if !in_comment {
                    words.push(parse_word(word))
                }
            }
        }
    }
    Ok(structure::Line { words, number })
}

pub fn parse_word(word: &str) -> structure::Word {
    let possnum = word.parse::<f64>();
    if possnum.is_ok() {
        let num = possnum.unwrap();
        return structure::Word::Number(match word.contains(".") {
            true => structure::Number::Float(num),
            false => structure::Number::Integer(num as i64),
        });
    } else {
        return structure::Word::Identifier(word.to_string());
    }
}
