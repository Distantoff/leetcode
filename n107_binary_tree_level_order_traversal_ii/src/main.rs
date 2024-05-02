fn main() {
    // let mut r0 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    // let mut l1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    // let mut r1 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    // let mut l2 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    // let mut r2 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    //
    //
    // r1.as_mut().unwrap().borrow_mut().left = l2;
    // r1.as_mut().unwrap().borrow_mut().right = r2;
    //
    // r0.as_mut().unwrap().borrow_mut().left = l1;
    // r0.as_mut().unwrap().borrow_mut().right = r1;
    let mut r0 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut l1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let mut r1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut l2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let mut r2 = Some(Rc::new(RefCell::new(TreeNode::new(5))));


    l1.as_mut().unwrap().borrow_mut().left = l2;
    r1.as_mut().unwrap().borrow_mut().right = r2;

    r0.as_mut().unwrap().borrow_mut().left = l1;
    r0.as_mut().unwrap().borrow_mut().right = r1;
    
    // dbg!(&r0);
    dbg!(Solution::level_order_bottom(r0));
    // dbg!(Solution::level_order_bottom(r0));
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut depth: usize) -> Vec<(usize, i32)> {
            let mut res = Vec::new();
            depth += 1;

            if let Some(ref l) = root.as_ref().unwrap().borrow().left {
                res = [res, dfs(&Some(l.clone()), depth)].concat();
            }

            if let Some(ref r) = root.as_ref().unwrap().borrow().right {
                res = [res, dfs(&Some(r.clone()), depth)].concat();
            }

            res.push((depth, root.as_ref().unwrap().borrow().val));
            res
        }

        if root.is_none() {
            return vec![];
        }

        let dfs_list = dfs(&root, 0);
        let mut res = vec![vec![]; *dfs_list.iter().map(|(depth, _)| depth).max().unwrap()];

        for item in &dfs_list {
            res[item.0 - 1].push(item.1);
        }

        res.reverse();
        res
    }
    
    // pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //     fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //         let mut res = vec![];
    //
    //         match (&root.as_ref().unwrap().borrow().left, &root.as_ref().unwrap().borrow().right) {
    //             (Some(ref l), Some(ref r)) => {
    //                 res = [res, dfs(&Some(l.clone())), dfs(&Some(r.clone()))].concat();
    //                 res.push(vec![l.borrow().val, r.borrow().val]);
    //                 res
    //             },
    //             (None, Some(ref r)) => {
    //                 res = [res, dfs(&Some(r.clone()))].concat();
    //                 res.push(vec![r.borrow().val]);
    //                 res
    //             },
    //             (Some(ref l), None) => {
    //                 res = [res, dfs(&Some(l.clone()))].concat();
    //                 res.push(vec![l.borrow().val]);
    //                 res
    //             },
    //             (None, None) => {
    //                 res
    //             },
    //         }
    //
    //     }
    //
    //     if root.is_none() {
    //         return vec![];
    //     }
    //
    //     [dfs(&root), vec![vec![root.as_ref().unwrap().borrow().val]]].concat()
    // }

    // fn dfs2(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //     let mut res = vec![];
    //
    //     res.insert(0, vec![root.as_ref().unwrap().borrow().val]);
    //
    //     let mut left_right: Vec<i32> = vec![];
    //
    //     if let Some(ref l) = root.as_ref().unwrap().borrow().left {
    //         left_right = dfs(&Some(l.clone())).into_iter().flatten().collect();
    //         // res = [res, dfs(&Some(l.clone()))].concat();
    //     }
    //
    //     if let Some(ref r) = root.as_ref().unwrap().borrow().right {
    //         left_right = [left_right, dfs(&Some(r.clone())).into_iter().flatten().collect()].concat();
    //         // res = [res, dfs(&Some(r.clone()))].concat();
    //     }
    //
    //     res
    // }
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

// 107. Binary Tree Level Order Traversal II
// Medium
// Topics
// Companies
// Given the root of a binary tree, return the bottom-up level order traversal of its nodes' values. (i.e., from left to right, level by level from leaf to root).
//
// Example 1:
// Input: root = [3,9,20,null,null,15,7]
// Output: [[15,7],[9,20],[3]]
//
// Example 2:
// Input: root = [1]
// Output: [[1]]
//
// Example 3:
// Input: root = []
// Output: []
//  
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 2000].
// -1000 <= Node.val <= 1000
