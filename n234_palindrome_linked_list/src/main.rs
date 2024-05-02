fn main() {
    let mut n1 = Some(Box::new(ListNode::new(1)));
    let mut n2 = Some(Box::new(ListNode::new(2)));
    let mut n3 = Some(Box::new(ListNode::new(2)));
    let mut n4 = Some(Box::new(ListNode::new(1)));
    let mut n5 = Some(Box::new(ListNode::new(7)));

    n3.as_mut().unwrap().next = n4;
    n5.as_mut().unwrap().next = n3;
    n2.as_mut().unwrap().next = n5;
    n1.as_mut().unwrap().next = n2;

    // n3.as_mut().unwrap().next = n4;
    // n2.as_mut().unwrap().next = n3;
    // n1.as_mut().unwrap().next = n2;

    dbg!(Solution::is_palindrome3(n1));
}

impl Solution {
    pub fn is_palindrome3(head: Option<Box<ListNode>>) -> bool {
        // dbg!(&head);
        let mut slow = head.clone();
        let mut fast = &head;
        let mut prev = None;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            let slow_temp = slow.as_ref().unwrap().next.clone();
            slow.as_mut().unwrap().next = prev;
            prev = slow;
            slow = slow_temp;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        if fast.is_some() {
            slow = slow.unwrap().next;
        }
        
        while prev.is_some() && slow.is_some() {
            if slow.as_ref().unwrap().val != prev.as_ref().unwrap().val {
                return false;
            }
            prev = prev.unwrap().next;
            slow = slow.unwrap().next;
        }

        true
    }
    // Рабочий вариант, свой и кстати работает доволно шустро
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut ref_node = &head;
        let mut len = 0;

        while let Some(ref node) = &ref_node {
            ref_node = &node.next;
            len += 1;
        }

        ref_node = &head;
        let mut i = 0;
        let mut num = 0;
        let mut skip_num = if len % 2 == 1 { len / 2 + 1 } else { 0 };

        while let Some(node) = ref_node {
            ref_node = &node.next;

            if skip_num > 0 && skip_num - 1 == i {
                skip_num = 0;
                continue;
            }

            if i < len / 2 {
                num = num * 10 + node.val as usize;
            } else {
                num -= node.val as usize * 10_usize.pow(i - len / 2);
            }
            i += 1;
        }

        num == 0
    }


    // Классное вроде бы решение, но увы переполнение,
    // когда слишком много узлов
    pub fn is_palindrome2(head: Option<Box<ListNode>>) -> bool {
        let mut ref_node = &head;
        let mut len = 0;
        let mut num = 0;

        while let Some(node) = ref_node {
            ref_node = &node.next;
            len += 1;
            num = num * 10 + node.val as usize;
        }

        let mut left_num = num;

        while len != 0 {
            let num_size = 10_usize.pow(len - 1);
            let n1 = left_num / num_size;
            let n2 = num % 10;
            println!("{num} {left_num}: {n1} - {n2}");
            num = num / 10;
            left_num = left_num % num_size;
            len -= 1;

            if n1 != n2 {
                return false;
            }
        }
        dbg!(&num);
        dbg!(&len);

        true
    }
}

struct Solution { }

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
