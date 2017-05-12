// Given an array of integers, return indices of the two numbers
// such that they add up to a specific target.
// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.
//
// Example:
//     Given nums = [2, 7, 11, 15], target = 9,
//     Because nums[0] + nums[1] = 2 + 7 = 9,
//     return [0, 1].

// 时间复杂度O(n)

//! # Examples
//!
//! ```
//! use algorithms::array::two_sum;
//! let v = vec![2, 7, 11, 15];
//! assert_eq!(two_sum::two_sum_3(v, 9), [0, 1]);
//! ```

pub fn two_sum_1(nums: Vec<usize>, target: usize) -> Vec<usize> {
    let mut v = vec![];
    for n in 0..nums.len()-1 {
        if nums[n] + nums[n+1] == target {
            v.push(n);
            v.push(n+1);
        }
    }
    v
}


pub fn two_sum_2(nums: Vec<usize>, target: usize) -> Vec<usize> {
    let mut v = vec![];
    for i in 0..nums.len() {

        // 比较典型的安全问题：usize类型，如果相减，有可能就是负数。
        // 此处报错：'attempt to subtract with overflow'
        // 所以使用checked_sub方法，当越界的时候返回None
        // 这里用unwrap_or，在越界的时候返回0，但是这可能造成逻辑bug
        let diff = target.checked_sub(nums[i]).unwrap_or(0);
        if nums.contains(&diff) {
            let idx = nums.iter().position(|&r| r == diff).unwrap();
            v.push(idx);
        }
    }
    v.sort();
    v
}

// change with match
pub fn two_sum_3(nums: Vec<usize>, target: usize) -> Vec<usize> {
    let mut v = vec![];
    for i in 0..nums.len() {
        let diff = target.checked_sub(nums[i]);
        match diff {
            Some(_) => if nums.contains(&diff.unwrap()) {
                let idx = nums.iter().position(|&r| r == diff.unwrap()).unwrap();
                v.push(idx);
            },
            _ => ()
        }

    }
    v.sort();
    v
}
