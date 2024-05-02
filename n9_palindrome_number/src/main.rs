fn main() {
    dbg!(Solution::is_palindrome(122));
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut tmp = x;
        let mut res = 0;

        while tmp != 0 {
            let remainder = tmp % 10;
            tmp /= 10;
            res = res * 10 + remainder;
        }

        res == x
    }
}

// slow
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x == 0 {
        return true;
    }

    let length = x.ilog10();
    let mut tmp = x;
    let mut res = 0;

    for i in (0..=length).rev() {
        let remainder = tmp % 10;
        tmp /= 10;
        res += remainder * 10_i32.pow(i);
    }
    
    res == x
}

struct Solution {}
