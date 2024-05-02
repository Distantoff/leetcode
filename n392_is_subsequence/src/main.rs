fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut cur_pos = 0;
        for ch in t.chars() {
            if s.len() > cur_pos && ch == s[cur_pos..=cur_pos].parse().unwrap() {
                cur_pos += 1;
            }
        }

        cur_pos == s.len()
    }
}

struct Solution { }

// TS
// function isSubsequence(s: string, t: string): boolean {
//     if (s.length == 0) {
//         return true;
//     }
//
//     let cur_pos = 0;
//     for (const ch of t) {
//         if (s.length > cur_pos && ch == s[cur_pos]) {
//             cur_pos++;
//         }
//     }
//
//     return cur_pos == s.length;
// };


// 392. Is Subsequence
// Solved
// Easy
// Topics
// Companies
// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//
// A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
//
// Example 1:
//
// Input: s = "abc", t = "ahbgdc"
// Output: true
// Example 2:
//
// Input: s = "axc", t = "ahbgdc"
// Output: false
//  
//
// Constraints:
//
// 0 <= s.length <= 100
// 0 <= t.length <= 104
// s and t consist only of lowercase English letters.
//  
//
// Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 109, and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
