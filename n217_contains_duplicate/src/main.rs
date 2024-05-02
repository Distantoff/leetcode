fn main() {
    // let nums = vec![1,2,3,4,5,1];
    // dbg!(Solution::contains_duplicate(nums));
    
    let num: i32 = 4;
    let num2: i32 = 3;
    let two = 2;
    let xor = num >> 1;
    // let b = num.as_bytes();
    // dbg!(&b);
    println!("{:2}: {:08b} {:08b}: {:08b}", num, num, num - 1, num & num - 1 );
    let is_power_of_two = format!("{:b}", num);
}

impl Solution {
    // Увы тоже нет интересного решения, без использования коллекций
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut or = 0;
        let mut xor = 0;
        let mut prev = 0;

        for num in nums {
            or |= num;
            xor ^= num;
            // dbg!(&xor);
            // dbg!(&or);
            // println!("{}: {:08b} | {:08b}: {}", num, num, or, or);
            println!("{}: {:08b} ^ {:08b}: {}", num, num, xor, xor);
        }
        // dbg!(&xor);
        false
    }
}

struct Solution { }

// Code
// Testcase
// Test Result
// Test Result
// 217. Contains Duplicate
// Easy
// Topics
// Companies
// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
//
//  
//
// Example 1:
//
// Input: nums = [1,2,3,1]
// Output: true
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: false
// Example 3:
//
// Input: nums = [1,1,1,3,3,4,3,2,4,2]
// Output: true
//  
//
// Constraints:
//
// 1 <= nums.length <= 105
// -109 <= nums[i] <= 109
