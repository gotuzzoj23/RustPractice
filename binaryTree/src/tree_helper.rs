use daily_tree::{Node};

    type NodeRef = Option<Box<Node>>;

    fn print_type_of<T>(_: &T) {
        println!("--> {}\n", std::any::type_name::<T>())
    }

    pub fn generate_tree(level: usize, counter: &mut i32) ->  NodeRef {
        if level == 0 {
            None
        } else {
            let mut node = Node {
                value: *counter,
                left: None,
                right: None,
            };
        *counter += 1;
        node.left = generate_tree(level - 1, counter);
        node.right = generate_tree(level - 1, counter);
        return Some(Box::new(node))
        }
    }

    pub fn print_tree(root: &NodeRef, level:usize) {
        match root {
            Some(node) => {
                print_tree(&node.left, level + 1);
                for _ in 0..level {
                    print!("   ");
                }
                println!("{}", node.value);
                print_tree(&node.right, level + 1);
            }
            None => {}
        }
    }
    
    pub fn invert_tree(root: NodeRef) -> NodeRef {
        match root {
            Some(node) => {
                Some(Box::new(Node{
                    value: node.value.clone(),
                    left: invert_tree(node.right),
                    right: invert_tree(node.left),
                }))
            }
            None => None
        }
    }
    
    