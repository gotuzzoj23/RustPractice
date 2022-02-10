use daily_tree::{Node};

    type NodeRef = Option<Box<Node>>;

    fn print_type_of<T>(_: &T) {
        println!("--> {}\n", std::any::type_name::<T>())
    }

    enum Action<T, U>{
        Call(T),
        Handle(U),
    }

    pub fn generate_tree_non_recur(mut level:  usize) -> NodeRef {
        let mut counter = 1;
        let mut arg_stack = Vec::<Action<usize,i32>>::new();
        let mut ret_stack = Vec::<NodeRef>::new();
        use Action::*;
        arg_stack.push(Call(level));
        while let Some(action) = arg_stack.pop() {
            match action {
                Call(level) => if level > 0 {
                    arg_stack.push(Handle(counter));
                    counter += 1;
                    arg_stack.push(Call(level - 1));
                    arg_stack.push(Call(level - 1));
                } else{
                    ret_stack.push(None);
                },
                Handle(value) => {
                    let left = ret_stack.pop().unwrap();
                    let right = ret_stack.pop().unwrap();
                    ret_stack.push(Some(Box::new(Node{value: value, left: left, right: right})));
                }
            }
        
        }
        ret_stack.pop().unwrap()
    }


    pub fn print_tree_non_recur(root: &NodeRef) {
        let mut stack = Vec::<Action<(&NodeRef, usize), (&i32, usize)>>::new();
        use Action::*;
        stack.push(Call((root, 0)));
        while let Some(action) = stack.pop() {
            match action {
                Call((root,level)) => if let Some(node) = root {
                    stack.push(Call((&node.right, level + 1)));
                    stack.push(Handle((&node.value, level)));
                    stack.push(Call((&node.left, level + 1)));
                },
                Handle((value, level)) => {
                    for _ in 0..level {
                        print!("  ");
                    }
                    println!("{}", value);
                }
            }
        }
    }

    pub fn invert_tree_non_recur(root: &NodeRef) -> NodeRef {
        let mut arg_stack = Vec::<Action<&NodeRef, i32>>::new();
        let mut ret_stack = Vec::<NodeRef>::new();

        use Action::*;
        arg_stack.push(Call(root));
        
        while let Some(action) = arg_stack.pop() {
            match action {
                Call(root) => if let Some(node) = root {
                    arg_stack.push(Handle(node.value));
                    arg_stack.push(Call(&node.right));
                    arg_stack.push(Call(&node.left));
                } else {
                    ret_stack.push(None);
                },
                Handle(value) => {
                    let left = ret_stack.pop().unwrap();
                    let right = ret_stack.pop().unwrap();
                    ret_stack.push(Some(Box::new(Node{value: value, left: left, right: right})));
                },
            }
        }
        ret_stack.pop().unwrap()
    }