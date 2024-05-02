fn main() {

    // let arr = vec![1,2,3,4,5,2,3,4,5];
    // dbg!(Solution::max_contains(arr));
    let num = 2;
    dbg!(Solution::is_happy(num));
}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;
        let mut n = n;
        let mut sum = 0;
        let mut prev_numbers: HashSet<i32> = HashSet::new();

        while n != 1 {
            sum = 0;
            while n > 0 {
                sum += (n % 10).pow(2);
                n /= 10;
            }

            n = sum;

            if !prev_numbers.insert(n) {
                return false;
            }
        }

        n == 1
    }


    pub fn is_happy_leetcode(mut n: i32) -> bool {
        loop {
            let mut s = 0;
            while n > 0 {
                s += (n % 10).pow(2);
                n /= 10;
            }
            match s {
                1 | 4 => break s == 1,
                _ => n = s,
            }
        }
    }


    pub fn max_contains(nums: Vec<usize>) -> usize {
        use std::collections::HashMap;
        let mut res = 0;
        let mut repeated: HashMap<usize, usize> = HashMap::new();
        repeated.insert(nums[0], 1);

        for i in 1..nums.len() {
            let num = &nums[i];
            if !repeated.contains_key(num) {
                repeated.insert(*num, repeated[&nums[i - 1]] + 1);
            } else {
                *repeated.get_mut(num).unwrap() = (i + 1) - repeated[&nums[i]];
            }

            res = res.max(repeated[&nums[i]]);
        }

        res
    }
}

struct Solution { }



// 202. Happy Number
// Easy
// Topics
// Companies
// Write an algorithm to determine if a number n is happy.
//
// A happy number is a number defined by the following process:
//
// Starting with any positive integer, replace the number by the sum of the squares of its digits.
// Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
// Those numbers for which this process ends in 1 are happy.
// Return true if n is a happy number, and false if not.
//
// Example 1:
// Input: n = 19
// Output: true
// Explanation:
// 12 + 92 = 82
// 82 + 22 = 68
// 62 + 82 = 100
// 12 + 02 + 02 = 1
//
// Example 2:
// Input: n = 2
// Output: false
//  
//
// Constraints:
//
// 1 <= n <= 231 - 1
