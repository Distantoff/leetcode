use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let (mut l0, mut r1, mut l1, l2) = (
       Some(Rc::new(RefCell::new(TreeNode::new(1)))),
       Some(Rc::new(RefCell::new(TreeNode::new(2)))),
       Some(Rc::new(RefCell::new(TreeNode::new(3)))),
       Some(Rc::new(RefCell::new(TreeNode::new(21)))),
    );
    r1.as_mut().unwrap().borrow_mut().left = l1.clone();
    l0.as_mut().unwrap().borrow_mut().right = r1.clone();

    dbg!(Solution::is_same_tree(l0.clone(), l0));
}

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        false
    }
}

struct Solution { }

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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
}

// Given the roots of two binary trees p and q, write a function to check if they are the same or not.
//
// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
//
// Example 1:
// Input: p = [1,2,3], q = [1,2,3]
// Output: true
//
// Example 2:
// Input: p = [1,2], q = [1,null,2]
// Output: false
//
// Example 3:
// Input: p = [1,2,1], q = [1,1,2]
// Output: false
//  
//
// Constraints:
//
// The number of nodes in both trees is in the range [0, 100].
// -104 <= Node.val <= 104
