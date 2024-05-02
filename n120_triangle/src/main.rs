fn main() {
    let triangle = vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]];
    // let triangle = vec![vec![-1],vec![2,3],vec![1,-1,-3]];
    // let triangle = vec![vec![-1],vec![3,2],vec![-3,1,-1]];
    dbg!(Solution::minimum_total(triangle));
}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = triangle.clone();
        for i in 0..triangle.len() - 1 {
            for j in 0..=i {
                dp[i + 1][j + 1] += dp[i][j];

                if j == 0 {
                    dp[i + 1][j] += dp[i][j];
                } else {
                    dp[i + 1][j] = (dp[i][j] + triangle[i + 1][j]).min(dp[i + 1][j]);
                }
            }
        }

        *dp.iter().last().unwrap().iter().min().unwrap()
    }
}

struct Solution { }

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let triangle = vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]];
        assert_eq!(11, Solution::minimum_total(triangle));
    }

    #[test]
    fn test2() {
        let triangle = vec![vec![-1],vec![3,2],vec![-3,1,-1]];
        assert_eq!(-1, Solution::minimum_total(triangle));
    }
}


// 120. Triangle
// Medium
// Topics
// Companies
// Given a triangle array, return the minimum path sum from top to bottom.
//
// For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.
//
//  
//
// Example 1:
//
// Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
// Output: 11
// Explanation: The triangle looks like:
//    2
//   3 4
//  6 5 7
// 4 1 8 3
// The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
// Example 2:
//
// Input: triangle = [[-10]]
// Output: -10
//  
//
// Constraints:
//
// 1 <= triangle.length <= 200
// triangle[0].length == 1
// triangle[i].length == triangle[i - 1].length + 1
// -104 <= triangle[i][j] <= 104
