#![allow(unused)]

fn main() {
    let mut nums = vec![0,1,0,3,12];
    let mut nums = vec![0,0,0,1,0,3,12];
    Solution::move_zeroes_leetcode(&mut nums);
    dbg!(&nums);

    // let num: usize = 7;
    // println!("{} {:08b}, {} {:08b}", num, num, num.pow(2), num.pow(2));
}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut first_zero_index = -1; 
        let mut offset = -1;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                first_zero_index = if first_zero_index == -1 { i as i32 } else { first_zero_index };
                offset += 1;
            } else if first_zero_index > -1 {
                nums.swap(first_zero_index as usize, i);
                first_zero_index = i as i32 - offset;
            }
        }
    }

    pub fn move_zeroes_leetcode(nums: &mut Vec<i32>) {
        let (mut ptr, len) = (0, nums.len());
        for i in 0..len {
            if nums[i] == 0 {
                continue;
            }
            nums.swap(ptr, i);
            ptr += 1;
        }

    }
}

struct Solution { }

// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
//
// Note that you must do this in-place without making a copy of the array.
//
// Example 1:
//
// Input: nums = [0,1,0,3,12]
// Output: [1,3,12,0,0]
// Example 2:
//
// Input: nums = [0]
// Output: [0]
//  
//
// Constraints:
//
// 1 <= nums.length <= 104
// -231 <= nums[i] <= 231 - 1
//  
// Follow up: Could you minimize the total number of operations done?
