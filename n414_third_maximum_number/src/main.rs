fn main() {
    // let nums = vec![3,2,1];
    let nums = vec![2,3,1,5];
    dbg!(Solution::third_max(nums));
}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let (mut max, mut middle, mut min) = (None, None, None);
        let mut length = 0;

        for num in nums {
            let num = Some(num);

            if num == max || num == middle || num == min {
                continue;
            }

            if num > max {
                (min, middle, max) = (middle, max, num);
            } else if num > middle {
                (min, middle) = (middle, num);
            } else if num > min {
                min = num;
            }

            length += 1;
        }

        if length < 3 { max.unwrap() } else { min.unwrap() }
    }
}

struct Solution { }


// TS
// function thirdMax(nums: number[]): number {
//     let min = Number.MIN_SAFE_INTEGER;
//     let middle = Number.MIN_SAFE_INTEGER;
//     let max = Number.MIN_SAFE_INTEGER;
//     let length = 0;
//
//     for (let num of nums) {
//         if (num == min || num == middle || num == max) {
//             continue;
//         }
//
//         if (num > max) {
//             min = middle;
//             middle = max;
//             max = num;
//         } else if (num > middle) {
//             min = middle;
//             middle = num;
//         } else if (num > min) {
//             min = num;
//         }
//
//         length++;
//     }
//
//     return length < 3 ? max : min;
// };

// 414. Third Maximum Number
// Easy
// Topics
// Companies
// Given an integer array nums, return the third distinct maximum number in this array. If the third maximum does not exist, return the maximum number.
//
// Example 1:
//
// Input: nums = [3,2,1]
// Output: 1
// Explanation:
// The first distinct maximum is 3.
// The second distinct maximum is 2.
// The third distinct maximum is 1.
// Example 2:
//
// Input: nums = [1,2]
// Output: 2
// Explanation:
// The first distinct maximum is 2.
// The second distinct maximum is 1.
// The third distinct maximum does not exist, so the maximum (2) is returned instead.
// Example 3:
//
// Input: nums = [2,2,3,1]
// Output: 1
// Explanation:
// The first distinct maximum is 3.
// The second distinct maximum is 2 (both 2's are counted together since they have the same value).
// The third distinct maximum is 1.
