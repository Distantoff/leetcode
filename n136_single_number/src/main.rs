fn main() {
    // let nums = vec![4,1,2,1,2];
    let nums = vec![1,4,2,1,2];
    dbg!(Solution::single_number(nums));
}

impl Solution {
    // Требоавалась сложность по памяти константная O(1), тут у меня линейная
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut repeated = vec![false; 60001];
        let mut sum = 0;

        for num in nums {
            let index = (num + 30000) as usize;
            repeated[index] = !repeated[index];
            sum = if repeated[index] == true { sum + num } else { sum - num };
        }

        sum
    }
}

struct Solution { }
