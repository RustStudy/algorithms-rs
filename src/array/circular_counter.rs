/*
There are people sitting in a circular fashion,
print every third member while removing them,
the next counter starts immediately after the member is removed.
Print till all the members are exhausted.

For example:
Input: consider 123456789 members sitting in a circular fashion,
Output: 369485271

首尾相接：

123456789 -> 3
12456789 -> 6
1245789 -> 9
124578 -> 4
12578 -> 8
1257 -> 5
127 -> 2
17 -> 7
1 -> 1

*/

//! # Examples
//!
//! ```
//! use algorithms::array::circular_counter;
//! let nums = vec![1,2,3,4,5,6,7,8,9];
//! assert_eq!(circular_counter::circular_counter_1(nums, 3), [3, 6, 9, 4, 8, 5, 2, 7, 1]);
//! ```


// 利用求余来计算周期
pub fn circular_counter_1(mut nums: Vec<usize>, mut period: usize) -> Vec<usize> {
    period = period - 1;
    let mut idx = 0;
    let mut v = vec![];
    while nums.len() > 0 {
        idx = (idx + period) % nums.len();  // 从idx开始的第( (idx + period) % nums.len() )个数被删除
        println!("{:?}", nums);
        println!("{:?}", idx);
        v.push(nums.remove(idx));
    }
    v
}
