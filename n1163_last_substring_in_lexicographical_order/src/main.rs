fn main() {
    let line = "leetcode";
    println!("The last substring is {}", Solution::last_substring(line.to_string()));
}

struct Solution {}

impl Solution {
    pub fn last_substring(s: String) -> String {
        let s = s.as_bytes();
        let mut res = &s[..];

        for (i, val) in s.iter().enumerate() {
            let max_val = if val > &s.iter().next().unwrap()
                && s[i..] > s[i + 1..] {
                &s[i..] } else { &s[i + 1..] };
            res = res.max(max_val);
        }

        unsafe { String::from_utf8_unchecked(res.to_vec()) }
    }
}

// Ссылка на моё решение:
// https://leetcode.com/problems/last-substring-in-lexicographical-order/solutions/4787569/simple-but-not-exactly-quick-solution/

// Given a string s, return the last substring of s in lexicographical order.
// Example 1:
// Input: s = "abab"
// Output: "bab"
// Explanation: The substrings are ["a", "ab", "aba", "abab", "b", "ba", "bab"]. The lexicographically maximum substring is "bab".
// Example 2:
//
// Input: s = "leetcode"
// Output: "tcode"
//  
//
// Constraints:
//
// 1 <= s.length <= 4 * 105
// s contains only lowercase English letters.
