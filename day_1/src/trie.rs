use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Trie {
    pub n: Vec<TrieNode>,
}

impl Trie {
    pub fn new<T>(i: &[T]) -> Self
    where
        T: AsRef<str>,
    {
        let mut v: Vec<TrieNode> = Vec::new();
        for s in i {
            let s = s.as_ref();
            if let Some(b) = s.bytes().next() {
                if let Some(n) = v.iter_mut().find(|t| t.v == b) {
                    n.fill_trie_node(&s.as_bytes()[1..])
                } else {
                    let mut node = TrieNode {
                        v: b,
                        n: Rc::new(RefCell::new(Vec::new())),
                    };
                    node.fill_trie_node(&s.as_bytes()[1..]);
                    v.push(node);
                }
            }
        }
        Self { n: v }
    }

    pub fn try_find(&self, hey_stack: &str) -> Option<u8> {
        // there are no numbers with less than three chars
        if hey_stack.len() < 3 {
            return None;
        }
        let b = hey_stack.as_bytes();
        // We found a possible trie branch
        if let Some(n) = self.n.iter().find(|t| t.v == b[0]) {
            return n.match_trie_node(&b[1..]);
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct TrieNode {
    pub v: u8,
    pub n: Rc<RefCell<Vec<NodeType>>>,
}

impl TrieNode {
    fn match_trie_node(&self, b: &[u8]) -> Option<u8> {
        let x = self.n.borrow();
        if let Some(n) = x.iter().find(|e| e.is_b(b[0])) {
            match n {
                NodeType::Node(n) => n.match_trie_node(&b[1..]),
                NodeType::Fin { v: _, p } => Some(*p),
            }
        } else {
            None
        }
    }

    fn fill_trie_node(&mut self, b: &[u8]) {
        let mut x = self.n.borrow_mut();
        if let Some(NodeType::Node(n)) = x.iter_mut().find(|e| {
            let NodeType::Node(n) = e else {
                panic!("We shall not find a end when constructing a trie")
            };
            n.v == b[0]
        }) {
            // We found a trie branch with the same prefix as us so we use that.
            n.fill_trie_node(&b[1..])
        }
        if b.len() == 2 {
            // Our syntax to create the trie is that the last byte is the numerical value
            x.push(NodeType::Fin {
                v: b[0],
                p: (b[1] as char).to_digit(10).unwrap() as u8,
            });
        } else {
            let mut new_node = TrieNode {
                v: b[0],
                n: Rc::new(RefCell::new(Vec::new())),
            };
            new_node.fill_trie_node(&b[1..]);
            x.push(NodeType::Node(new_node));
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Node(TrieNode),
    Fin { v: u8, p: u8 },
}

impl NodeType {
    pub fn is_b(&self, b: u8) -> bool {
        match self {
            NodeType::Node(n) => n.v == b,
            NodeType::Fin { v, p: _ } => *v == b,
        }
    }
}
