fn main() {
    dbg!(Solution::generate(5));
}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut res = vec![vec![]; num_rows];
        for i in 0..num_rows {
            res[i] = vec![1; i + 1];

            if i > 1 {
                for j in 1..i {
                    res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
                }
            }
        }

        res
    }
}

struct Solution { }

// 118. Pascal's Triangle
// Easy
// Topics
// Companies
// Given an integer numRows, return the first numRows of Pascal's triangle.
//
// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
//
// Example 1:
// Input: numRows = 5
// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
//
// Example 2:
// Input: numRows = 1
// Output: [[1]]
//
// Constraints:
//
// 1 <= numRows <= 30
