fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();
    let s = "".to_string();
    dbg!(Solution::is_palindrome(s));
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s = s.to_lowercase();
        s.retain(|c| c.is_alphanumeric());

        for i in 0..s.chars().count() / 2 {
            if s[i..=i].parse::<char>().unwrap() != s.pop().unwrap() {
                return false;
            } 
        }

        true
    }
}

struct Solution { }
