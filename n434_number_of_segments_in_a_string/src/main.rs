fn main() {
    // Понятно что с помощью встроенных функций split_whitespace это можно сделать в одну строку
    let s = "Hello, my name is John".to_string();
    dbg!(Solution::count_segments(s));
}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut segments = 0;
        let mut is_prev_space = false;

        for (i, ch) in s.chars().enumerate() {
            if ch != ' ' && (is_prev_space == true || i == 0) {
                segments += 1;
            }
            is_prev_space = ch == ' ';
        }

        segments
    }
}

struct Solution { }

// 434. Number of Segments in a String
// Solved
// Easy
// Topics
// Companies
// Given a string s, return the number of segments in the string.
//
// A segment is defined to be a contiguous sequence of non-space characters.
//
//  
//
// Example 1:
//
// Input: s = "Hello, my name is John"
// Output: 5
// Explanation: The five segments are ["Hello,", "my", "name", "is", "John"]
// Example 2:
//
// Input: s = "Hello"
// Output: 1
//  
//
// Constraints:
//
// 0 <= s.length <= 300
// s consists of lowercase and uppercase English letters, digits, or one of the following characters "!@#$%^&*()_+-=',.:".
// The only space character in s is ' '.
