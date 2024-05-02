use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let (mut l0, mut r1, mut l1, l2) = (
       Some(Rc::new(RefCell::new(TreeNode::new(1)))),
       Some(Rc::new(RefCell::new(TreeNode::new(2)))),
       Some(Rc::new(RefCell::new(TreeNode::new(3)))),
       Some(Rc::new(RefCell::new(TreeNode::new(21)))),
    );
    r1.as_mut().unwrap().borrow_mut().left = l1;
    // l1.as_mut().unwrap().borrow_mut().left = l2;
    l0.as_mut().unwrap().borrow_mut().right = r1;
    dbg!(&l0);
    dbg!(Solution::inorder_traversal(l0));
}

impl Solution {

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut nodes = vec![(root, false)];
        let mut res: Vec<i32> = Vec::new();

        while let Some((node, is_can_be_added)) = nodes.pop() {
            if is_can_be_added {
                res.push(node.unwrap().borrow().val);
            } else if let Some(n) = node {
                nodes.push((n.borrow().right.clone(), false));
                nodes.push((Some(n.clone()), true));
                nodes.push((n.borrow().left.clone(), false));
            }
        }

        res
    }

    // Работает, но я не учел метод обхода inorder
    pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;

        let mut res: Vec<i32> = Vec::new();
        let mut viewed: Vec<i32> = Vec::new();
        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root);

        while let Some(Some(node)) = deque.pop_front() {
            if viewed.contains(&node.as_ref().borrow().val) {
                continue;
            }

            viewed.push(node.as_ref().borrow().val);
            res.push(node.as_ref().borrow().val);

            if node.as_ref().borrow().left.is_some() {
                deque.push_front(node.borrow_mut().left.to_owned());
            }

            if node.as_ref().borrow().right.is_some() {
                deque.push_back(node.borrow_mut().right.to_owned());
            }
        }

        res
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


// Given the root of a binary tree, return the inorder traversal of its nodes' values.
//
// Example 1:
// Input: root = [1,null,2,3]
// Output: [1,3,2]
//
// Example 2:
// Input: root = []
// Output: []
//
// Example 3:
// Input: root = [1]
// Output: [1]
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
//  
//
// Follow up: Recursive solution is trivial, could you do it iteratively?
