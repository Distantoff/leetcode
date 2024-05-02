fn main() {
    let s = "egg".to_string();
    let t = "asd".to_string();

    // [1,3,3]
    // [2,6,6]

    dbg!(Solution::is_isomorphic(s, t));
}

impl Solution {
    // Судя по всему элегантного решения нет, только с несколькими коллекциями
    pub fn is_isomorphic(mut s: String, mut t: String) -> bool {
        use std::collections::HashMap;
        let mut map: HashMap<char, char> = HashMap::new();
        
        while let Some(ch1) = s.pop() {
            let ch2 = t.pop().unwrap();

            if !map.contains_key(&ch1) {
                map.insert(ch1, ch2);
            } else if map[&ch1] != ch2 {
                return false;
            }
        }

        true
    }
}

struct Solution { }

// 205. Isomorphic Strings
// Easy
// Topics
// Companies
// Given two strings s and t, determine if they are isomorphic.
//
// Two strings s and t are isomorphic if the characters in s can be replaced to get t.
//
// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
//
//  
//
// Example 1:
//
// Input: s = "egg", t = "add"
//     Output: true
// Example 2:
//
// Input: s = "foo", t = "bar"
//     Output: false
// Example 3:
//
// Input: s = "paper", t = "title"
//     Output: true
//  
//
// Constraints:
//
// 1 <= s.length <= 5 * 104
// t.length == s.length
// s and t consist of any valid ascii character.
