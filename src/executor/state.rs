use crate::parser::structure::Number;
pub use crate::parser::structure::Word;

pub struct InterpreterState {
    pub words: Vec<WordDefinition>,
    pub variables: Vec<Variable>,
    pub constants: Vec<Variable>,
    pub stack: Vec<ListNode>,
}

// Lists that contain executable code, for example lists being used as closures,
// are represented by lists that have a Nil value, and in the children lists are
// either the sub lists that are contained in that code, such as the usage of
// more closures, or Lists that have no children, with the value the word to be
// executed
#[derive(Debug)]
pub struct List {
    pub children: Vec<List>,
    pub value: ListNode,
}

#[derive(Debug)]
pub enum ListNode {
    Atom(Atom),
    Nil,
}

#[derive(Debug)]
pub enum Atom {
    Float(f64),
    Integer(i64),
    Word(String),
}

impl From<Number> for Atom {
    fn from(num: Number) -> Self {
        match num {
            Number::Float(f) => Atom::Float(f),
            Number::Integer(i) => Atom::Integer(i),
            _ => panic!("Unreachable"),
        }
    }
}

pub struct WordDefinition {
    pub name: String,
    pub definition: List,
}

pub struct Variable {
    pub name: String,
    pub value: List,
}
