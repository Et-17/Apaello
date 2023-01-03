use super::state;
use crate::parser::structure;

// This function converts a given segment of words to an executable List
pub fn pack_words(words: &[structure::Word]) -> state::List {
    let mut packed = Vec::new();
    // A stack that maintains the sublist nesting
    let mut sub_lists: Vec<Vec<state::List>> = Vec::new();

    for i in words {
        match i {
            structure::Word::Number(n) => push_number(n, &mut sub_lists, &mut packed),
            structure::Word::Identifier(id) => push_identifier(id, &mut sub_lists, &mut packed),
        }
    }

    return state::List {
        children: packed,
        value: state::ListNode::Nil,
    };
}

pub fn push_number(
    num: &structure::Number,
    sub_lists: &mut Vec<Vec<state::List>>,
    packed: &mut Vec<state::List>,
) {
    let sub_lists_len = sub_lists.len();
    if sub_lists_len == 0 {
        packed.push(state::List {
            children: Vec::new(),
            value: state::ListNode::Atom(state::Atom::from(num.clone())),
        });
    } else {
        sub_lists[sub_lists_len - 1].push(state::List {
            children: Vec::new(),
            value: state::ListNode::Atom(state::Atom::from(num.clone())),
        })
    }
}

pub fn push_identifier(
    id: &str,
    sub_lists: &mut Vec<Vec<state::List>>,
    packed: &mut Vec<state::List>,
) {
    if id == "[" {
        sub_lists.push(Vec::new());
    } else if id == "]".to_string() {
        let finished_list = state::List {
            children: sub_lists.pop().unwrap(),
            value: state::ListNode::Nil,
        };

        if sub_lists.is_empty() {
            packed.push(finished_list);
        } else {
            // Avoid simultanious immutable ref and mutable ref
            let sub_lists_length = sub_lists.len();
            sub_lists[sub_lists_length - 1].push(finished_list);
        }
    } else {
        // Avoid simultanious immutable ref and mutable ref
        let sub_lists_length = sub_lists.len();
        if sub_lists_length == 0 {
            packed.push(state::List {
                children: Vec::new(),
                value: state::ListNode::Atom(state::Atom::Word(id.to_string())),
            });
        } else {
            sub_lists[sub_lists_length - 1].push(state::List {
                children: Vec::new(),
                value: state::ListNode::Atom(state::Atom::Word(id.to_string())),
            })
        }
    }
}
