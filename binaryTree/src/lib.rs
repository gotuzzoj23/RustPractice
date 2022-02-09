//use std::fmt::Debug;
type NodeRef = Option<Box<Node>>;

#[derive(Debug, Default, Clone)]
pub struct Node{
    pub value: i32,
    pub left: NodeRef,
    pub right: NodeRef,
}

impl Node {
    pub fn new(val: i32) -> Node{
        Node{
            value: val,
            left: None,
            right: None,
        }
    }

    pub fn print_tree(&self) {
        match &self.left {
            Some(node) => {
                node.print_tree();
            }
            None => {}
        }
        println!("{:?}", self.value);
        match &self.right {
            Some(node) => {
                node.print_tree();
            }
            None => {}
        }
    }

    pub fn insert(&mut self, value: i32) {
        let new_node = Some(Box::new(Node::new(value)));
        if value < self.value {
            match self.left.as_mut() {
                None => self.left = new_node,
                Some(left) => left.insert(value),
            }
        } else {
            match self.right.as_mut() {
                None => self.right = new_node,
                Some(right) => right.insert(value),
            }
        }
    }
}

