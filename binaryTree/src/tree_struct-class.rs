use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{LinkedList};

// Definition for a binary tree node.
 #[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn insert(&mut self, new_val: i32) {
        if self.val == new_val {
            return
        }
        let target_node = if new_val < self.val { &mut self.left } else { &mut self.right };
        match target_node {
            &mut Some(ref mut subnode) => subnode.as_ref().borrow_mut().insert(new_val),
            &mut None => {
                let new_node = TreeNode { val: new_val, left: None, right: None };
                let c = RefCell::new(new_node);
                let five = Rc::new(c);
                let option_node = Some(five);
                *target_node = option_node;
            }
        }
    }

    pub fn print_preorder(&self) {
        print!("{}, ", self.val);
        if !self.left.is_none() {
            self.left.as_ref().unwrap().borrow_mut().print_preorder();
        }
        if !self.right.is_none() {
            self.right.as_ref().unwrap().borrow_mut().print_preorder();
        }        
    }
    pub fn print_inorder(&self) {
        if !self.left.is_none() {
            self.left.as_ref().unwrap().borrow_mut().print_inorder();
        }
        print!("{}, ", self.val);
        if !self.right.is_none() {
            self.right.as_ref().unwrap().borrow_mut().print_inorder();
        }        
    }
    pub fn print_postorder(&self) {
        if !self.left.is_none() {
            self.left.as_ref().unwrap().borrow_mut().print_postorder();
        }
        if !self.right.is_none() {
            self.right.as_ref().unwrap().borrow_mut().print_postorder();
        }
        print!("{}, ", self.val);        
    }

    pub fn tree_to_list<'a>(&mut self, list: &'a mut LinkedList<i32>) -> &'a mut LinkedList<i32> {
        if !self.left.is_none() {
            self.left.as_ref().unwrap().borrow_mut().tree_to_list(list);
        }
        list.push_back(self.val);
        if !self.right.is_none() {
            self.right.as_ref().unwrap().borrow_mut().tree_to_list(list);
        }
        return list 
    }

    pub fn find(self, value: i32) -> Option<TreeNode> {
        if self.val == value {
            return Some(self)
        }
        if self.val > value {
            match self.left {
				None => None,
				Some(node) => node.as_ref().borrow_mut().find(value)
			}
        } else {
            match self.right {
				None => None,
				Some(node) => node.as_ref().borrow_mut().find(value)
			}
        }
    }

}

