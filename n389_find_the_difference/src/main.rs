fn main() {
    let s = "abcd".to_string();
    let t = "abcde".to_string();
    assert_eq!(Solution::find_the_difference(s, t), 'e');
}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut chars: u8 = 0;
        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..t.len() {
            chars ^= t[i];
            if i < s.len() {
                chars ^= s[i];
            }
        }

        chars as char
    }
}

struct Solution { }

// TS
// function findTheDifference(s: string, t: string): string {
//     let chars = 0;
//     for (let i = 0; i < t.length; i++) {
//         chars ^= t[i].charCodeAt(0);
//         if (i < s.length) {
//             chars ^= s[i].charCodeAt(0);
//         }
//     }
//
//     return String.fromCharCode(chars);
// };

// You are given two strings s and t.
//
// String t is generated by random shuffling string s and then add one more letter at a random position.
//
// Return the letter that was added to t.
//
//  
//
// Example 1:
//
// Input: s = "abcd", t = "abcde"
// Output: "e"
// Explanation: 'e' is the letter that was added.
// Example 2:
//
// Input: s = "", t = "y"
// Output: "y"
//  
//
// Constraints:
//
// 0 <= s.length <= 1000
// t.length == s.length + 1
// s and t consist of lowercase English letters.