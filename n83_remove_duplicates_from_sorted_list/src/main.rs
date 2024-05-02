fn main() {
    println!("Hello, world!");
    let (mut l1, mut l2, mut l3, mut l4, l5) = (ListNode::new(1), ListNode::new(2), ListNode::new(2), ListNode::new(3), ListNode::new(4));
    l4.next = Some(Box::new(l5));
    l3.next = Some(Box::new(l4));
    l2.next = Some(Box::new(l3));
    l1.next = Some(Box::new(l2));

    dbg!(&l1);
    dbg!(Solution::delete_duplicates(Some(Box::new(l1))));
}


impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut cur_node = Some(Box::new(ListNode::new(head.as_ref().unwrap().val)));
        let mut cur = &mut cur_node;
        let mut next_node = &head.as_ref().unwrap().next;

        loop {
            if *next_node == None {
                break;
            }

            if cur.as_ref().unwrap().val < next_node.as_ref().unwrap().val {
                cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(next_node.as_ref().unwrap().val)));
                cur = &mut cur.as_mut().unwrap().next;
            }

            next_node = &next_node.as_ref().unwrap().next;
        }

        cur_node
    }
}

struct Solution { }

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
