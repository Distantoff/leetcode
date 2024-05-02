use std::rc::Rc;
use std::cell::RefCell;

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

    // dbg!(&r0);

    dbg!(Solution::is_symmetric2(r0));
    // dbg!(Solution::is_symmetric_recursive2(r0));
}
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        let mut nodes = vec![(right, false), (left, true)];
        let mut left_nodes: Vec<_> = Vec::new();

        while let Some((node, is_left_side)) = nodes.pop() {
            if is_left_side {
                if let Some(ref n) = node {
                    nodes.push((n.borrow().left.clone(), is_left_side));
                    nodes.push((n.borrow().right.clone(), is_left_side));
                }

                left_nodes.insert(0, node.clone());
            } else {
                if let Some(ref n) = node {
                    nodes.push((n.borrow().right.clone(), is_left_side));
                    nodes.push((n.borrow().left.clone(), is_left_side));
                }

                match (node, left_nodes.pop()) {
                    (Some(r), Some(Some(l))) if r.borrow().val == l.borrow().val => continue,
                    (None, Some(None)) => continue,
                    _ => return false 
                }
            }
        }

        left_nodes.is_empty()
    }

    pub fn is_symmetric2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        let mut nodes = vec![(left, right)];

        while let Some(tuple) = nodes.pop() {
            match (tuple.0, tuple.1) {
                (Some(l), Some(r)) => {
                    if l.borrow().val != r.borrow().val {
                        return false;
                    }
                    nodes.push((l.borrow().left.clone(), r.borrow().right.clone()));
                    nodes.push((l.borrow().right.clone(), r.borrow().left.clone()));
                },
                (None, None) => continue,
                _ => return false
            };
        }

        true
    }


    // Мое решение, я сперва сделал массивы, обойдя левый и парвый узлы.
    // Следующее решение более рационально, там просто сравниваем левые
    // и правые узлы, каждого узла.
    pub fn is_symmetric_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recursive(root: &Option<Rc<RefCell<TreeNode>>>, is_left_side: bool) -> Vec<Option<i32>> {
            if root.is_none() {
                return vec![None];
            }

            let mut res = vec![Some(root.as_ref().unwrap().borrow().val)];
            let mut left = &root.as_ref().unwrap().borrow().left;
            let mut right = &root.as_ref().unwrap().borrow().right;

            if !is_left_side {
                (left, right) = (right, left);
            }

            res = [res, recursive(left, is_left_side)].concat();
            res = [res, recursive(right, is_left_side)].concat();

            res
        }

        let left = &root.as_ref().unwrap().borrow().left;
        let right = &root.as_ref().unwrap().borrow().right;
        let left_vec = recursive(left, true); 
        let right_vec = recursive(right, false); 

        left_vec == right_vec
    }

    pub fn is_symmetric_recursive2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(left: &Node, right: &Node) -> bool {
            match(left, right) {
                (None, None) => true,
                (Some(l), Some(r)) => {
                    l.borrow().val == r.borrow().val
                        && dfs(&l.borrow().left, &r.borrow().right)
                        && dfs(&l.borrow().right, &r.borrow().left)
                },
                _ => false
            }
        }

        let left = &root.as_ref().unwrap().borrow().left;
        let right = &root.as_ref().unwrap().borrow().right;

        dfs(&left, &right)
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


// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
//
// Example 1:
// Input: root = [1,2,2,3,4,4,3]
// Output: true
//
// Example 2:
// Input: root = [1,2,2,null,3,null,3]
// Output: false
//  
// Constraints:
//
// The number of nodes in the tree is in the range [1, 1000].
// -100 <= Node.val <= 100
//  
//
// Follow up: Could you solve it both recursively and iteratively?
