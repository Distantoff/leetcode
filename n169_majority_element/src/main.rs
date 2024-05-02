#[allow(unused)]

fn main() {
    // let nums = vec![2,2,1,1,1,2,2];
    // let nums = vec![3,2,3];
    // let nums = vec![2,2];
    let nums = vec![3,3,4];
    let nums = vec![2,3,3,4,3];
    dbg!(Solution::majority_element(nums));
}

impl Solution {
    // Boyer-Moore Majority Voting Algorithm
    // https://www.geeksforgeeks.org/boyer-moore-majority-voting-algorithm/
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let count = 0;
        let condidate = nums[0];

        for num in nums {
            if count == 0 {
                condidate = num;
            }

            if condidate == num {
                count += 1;
            } else {
                count -= 1;
            }
        }

        condidate
    }
}

struct Solution { }
