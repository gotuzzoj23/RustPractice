use std::io::{stdin};
use daily_tree::{Node};
mod tree_helper_recursive;
mod tree_helper_non_recursive;

fn main() {
    println!("Hello, world!");
    /*
    let x = fibonacci_recursive(10);
    println!("{}", x);
    */
    tree_fn();
}

fn print_type_of<T>(_: &T) {
    println!("--> {}\n", std::any::type_name::<T>())
}

fn get_usr_str() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Did not enter correct string!!!");
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    return s
}

fn str_to_vec(str: String) -> Vec<i32> {
    let vec_out: Vec<i32> =
        str.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(); 
    return vec_out
}

fn fibonacci_recursive(n: i64) -> i64 {
    if n < 2 {
        return n;
    }
    return fibonacci_recursive(n - 1) + fibonacci_recursive( n - 2);
}

fn  create_tree() -> Node {
        let mut tree = Node::new(10); 
        tree.insert(5);
        tree.insert(15);
        tree.insert(1);
        tree.insert(12);
        tree.insert(0);
        return tree;
}

fn tree_fn() {
    //let mut tree = create_tree();
    use tree_helper_recursive::*;
    let mut counter = 1;
    let tree = generate_tree(3, &mut counter);
    println!("=======Recursive======");
    tree.as_ref().unwrap().print_tree();
    println!("======================");
    print_tree(&tree, 0);
    println!("======Inverted========");
    let inverted_tree = invert_tree(tree);
    print_tree(&inverted_tree, 0);
    println!("\n");
    
    println!("=========Non==========");
    println!("======Recursive=======");
    use tree_helper_non_recursive::*;
    let mut x = 3;
    let tree_non_recur = generate_tree_non_recur(x);
    print_tree_non_recur(&tree_non_recur);
    println!("=======Inverted=======");
    print_tree_non_recur(&invert_tree_non_recur(&tree_non_recur));
    println!("======================");

    
}



