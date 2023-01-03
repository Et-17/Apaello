#[derive(Debug, PartialEq)]
pub enum Word {
    Number(Number),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct Line {
    pub words: Vec<Word>,
    pub number: usize,
}

#[derive(Debug)]
pub struct File {
    pub lines: Vec<Line>,
}
