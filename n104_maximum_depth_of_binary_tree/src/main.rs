fn main() {
    let mut r0 = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    let mut l1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let mut ll2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut lr2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    let mut r1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let mut rl2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let mut rr2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    l1.as_mut().unwrap().borrow_mut().left = ll2;
    l1.as_mut().unwrap().borrow_mut().right = lr2;

    r1.as_mut().unwrap().borrow_mut().left = rl2;
    r1.as_mut().unwrap().borrow_mut().right = rr2;

    r0.as_mut().unwrap().borrow_mut().left = l1;
    r0.as_mut().unwrap().borrow_mut().right = r1;
    
    dbg!(Solution::max_depth(r0));
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if root.is_none() {
                return 0;
            }

            1 + dfs(&root.as_ref().unwrap().borrow().left)
                .max(dfs(&root.as_ref().unwrap().borrow().right))
        }

        dfs(&root)
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

// Easy
// Topics
// Companies
// Given the root of a binary tree, return its maximum depth.
//
// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
//
// Example 1:
// Input: root = [3,9,20,null,null,15,7]
// Output: 3
//
// Example 2:
// Input: root = [1,null,2]
// Output: 2
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 104].
// -100 <= Node.val <= 100
