fn main() {
    // let num = 36;
    // println!("{} {:08b} {:08b} {:08b} {}", num, num, -num, num & -num, num & -num);
    let (num1, num2) = (4, 5);
    println!("{:08b} {:08b} {:08b}", num1, num2, num1 ^ num2);
    let nums = vec![1,2,1,3,2,5];
    // dbg!(&nums);
    dbg!(Solution::single_number(nums));

}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut acc = 0;
        // XOR операцией убираем дубликаты, и оставляем
        // уникальные числа, получая a XOR b
        for num in &nums {
            // println!("{acc:08b} {num:08b} {:08b}", acc ^ num);
            acc ^= *num;
        }

        // Такой операцией получаем младший битсет числа,
        acc &= -acc;

        // println!("{} {:08b} {:08b} {:08b}", acc, acc, -acc as u8, acc & -acc);

        let mut res = vec![0, 0];
        for num in &nums {
            // println!("{} {:08b} {:08b}", num, num,  acc & num);
            // Интересная вещь, что младший битсет результата XOR
            // двух уникальных чисел, с операцией AND - у одного
            // числа будет ноль и у другого не ноль.
            // Таким образом, мы можем разделить числа друг от друга
            // Ну и опять используя XOR мы убираем дубликаты
            if num & acc == 0 {
                res[0] ^= num;
            } else {
                res[1] ^= num;
            }
        }

        res
    }
}

struct Solution { }

// Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.
//
// You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.
// Example 1:
//
// Input: nums = [1,2,1,3,2,5]
// Output: [3,5]
// Explanation:  [5, 3] is also a valid answer.
// Example 2:
//
// Input: nums = [-1,0]
// Output: [-1,0]
// Example 3:
//
// Input: nums = [0,1]
// Output: [1,0]
//  
//
// Constraints:
//
// 2 <= nums.length <= 3 * 104
// -231 <= nums[i] <= 231 - 1
// Each integer in nums will appear twice, only two integers will appear once.
