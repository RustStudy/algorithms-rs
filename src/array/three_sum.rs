/*
    Given an array S of n integers, are there elements a, b, c in S
    such that a + b + c = 0?
    Find all unique triplets in the array which gives the sum of zero.
    Note: The solution set must not contain duplicate triplets.
    For example, given array S = [-1, 0, 1, 2, -1, -4],
    A solution set is:
    [
      [-1, 0, 1],
      [-1, -1, 2]
    ]

衍生:
    排列算法：
    1. 字典序
    2. 递增进位制数
    3. 递减进位制数
    4. 邻位对换
    5. 递归

    组合算法：
    1. 二进制转化法
    2. 递归
    3. 01转换法

*/

//! # Examples
//!
//! ```
//! use algorithms::array::three_sum;
//! let v = vec![-1, 0, 1, 2, -1, -4];
//! assert_eq!(three_sum::three_sum_1(v), [(-1, -1, 2), (-1, 0, 1)]);
//! ```

pub fn three_sum_1(mut nums: Vec<isize>) -> Vec<(isize, isize, isize)> {
    let mut res = vec![];
    nums.sort(); // need nums mutable: [-4, -1, -1, 0, 1, 2]
    for i in 0..(nums.len() - 2) {
        if i > 0 && nums[i] == nums[i-1] {
            continue;
        }
        let mut l = i+1;
        let mut r = nums.len() - 1;

        while l < r {
            let s = nums[i] + nums[l] + nums[r];
            if s > 0{
                r -= 1;
            }else if s < 0 {
                l += 1;
            }else{
                res.push((nums[i], nums[l], nums[r]));
                while l < r && nums[l] == nums[l+1]{
                    l += 1;
                }
                while l < r && nums[r] == nums[r-1]{
                    r -= 1;
                }
                l += 1;
                r -= 1;
            }
        }
    }
    res
}
