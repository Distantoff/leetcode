fn main() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];

    let mut nums1 = vec![0,0,0];
    let mut nums2 = vec![1];

    // let mut nums1 = vec![-1,0,0,3,3,3,0,0,0];
    // let mut nums2 = vec![1,2,2];

    let mut nums1 = vec![0];
    let mut nums2 = vec![1];

    let mut nums1 = vec![0, 1, 2, 0, 0, 0];
    let mut nums2 = vec![3, 4, 5];
    Solution::merge2(&mut nums1, 3, &mut nums2, 3);
    dbg!(&nums1);
    dbg!(&nums2);
}

// Мое собственное рабочее решение
// Интересная задачка, ключевым в ней то, что сперва мы сравниваем
// элементы, а когда справа уже больше, то сравниваем
// длину массивов, и просто меняем значение на значение из второго
// массива. В этом случе обязательно ведем смещение, ведь
// первоначальный массив меняется по длине, если перед
// этим было удачное сравнение значений элементов массива
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut offset = m as usize;
        for i in 0..(m + n) as usize {

            match (nums1.get(i), nums2.get(0)) {
                (Some(n1), Some(n2)) if n1 > n2 => {
                    nums1.pop();
                    nums1.insert(i, nums2.remove(0));
                    offset += 1;
                },
                (Some(_), Some(_)) if i >= offset => {
                    *nums1.get_mut(i).unwrap() = nums2.remove(0);
                },
                _ => { continue; }
            }
        }
    }


    // let mut nums1 = vec![1,2,3,9,0,0,0,0];
    // let mut nums2 = vec![2,5,6,7];
    pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);

        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }
}

struct Solution { }
