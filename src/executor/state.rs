pub use crate::parser::structure::Word;

pub struct InterpreterState {
    pub words: Vec<WordDefinition>,
    pub variables: Vec<Variable>,
    pub constants: Vec<Variable>,
    pub stack: Vec<ListNode>,
}

pub struct List {
    pub next: Option<Box<List>>,
    pub value: ListNode,
}

pub enum ListNode {
    Atom(Atom),
    SubList(Box<List>),
    Empty
}

pub enum Atom {
    Float(f64),
    Integer(i64),
}

pub struct WordDefinition {
    pub name: String,
    pub definition: Vec<Word>,
}

pub struct Variable {
    pub name: String,
    pub value: List,
}