fn main() {
    dbg!(Solution::climb_stairs2(7));
}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (2..=n).fold((1, 1), |acc, _| (acc.1, acc.0 + acc.1)).1
    }

    pub fn climb_stairs3(n: i32) -> i32 {
        let mut ways_count = 1;
        for i in 1..=(n / 2) {
            ways_count += n - i;
        }
        ways_count -= if n == 4 { 1 } else { 0 };
        ways_count
    }
}

// Leetcode
impl Solution {
    pub fn climb_stairs2(n: i32) -> i32 {
       let (mut a, mut b) = (1, 1);
        for _ in 2..=n {
            (a, b) = (b, a + b);
        }
    }
}

struct Solution {}
