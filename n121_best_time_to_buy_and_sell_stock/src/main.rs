fn main() {
    // let prices = vec![2,1,7,4,6,3]; // 6
    let prices = vec![7,1,5,3,6,4]; // 5
    // let prices = vec![2,1,2,1,0,0,1]; // 1

    dbg!(Solution::max_profit(prices));
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut min, mut res) = (10000, 0);

        for price in prices {
            res = res.max(price - min);
            min = min.min(price);
        }

        res
    }
}

struct Solution { }
