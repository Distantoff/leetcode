fn main() {
    let arr = vec![1,2,2,1,1,3];
    dbg!(Solution::unique_occurrences(arr));
}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut number_occurrences: HashMap<i32, u32> = HashMap::new();

        for num in arr {
            *number_occurrences.entry(num).or_insert(0) += 1;
        }

        let len_of_unique_numbers = number_occurrences.len();
        let unique_occurrences: HashSet<_> = number_occurrences.into_values().collect();
        unique_occurrences.len() == len_of_unique_numbers
    }
}

struct Solution {}


// Given an array of integers arr, return true if the number of occurrences of each value in the array is unique or false otherwise.
// Example 1:
//
// Input: arr = [1,2,2,1,1,3]
// Output: true
// Explanation: The value 1 has 3 occurrences, 2 has 2 and 3 has 1. No two values have the same number of occurrences.
// Example 2:
//
// Input: arr = [1,2]
// Output: false
// Example 3:
//
// Input: arr = [-3,0,1,-3,1,1,1,-3,10,0]
// Output: true
//  
//
// Constraints:
//
// 1 <= arr.length <= 1000
// -1000 <= arr[i] <= 1000
