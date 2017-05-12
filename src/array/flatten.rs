/*
Implement Flatten Arrays.
Given an array that may contain nested arrays,
give a single resultant array.
function flatten(input){
}
Example:
Input: var input = [2, 1, [3, [4, 5], 6], 7, [8]];
flatten(input);
Output: [2, 1, 3, 4, 5, 6, 7, 8]
*/
// 在Rust表示类似于示例中的多维数组比较困难，所以简化一下使用固定的二维数组
//
//! # Examples
//!
//! ```
//! use algorithms::array::flatten;
//! let v = vec![vec![1,2,3], vec![4, 5]];
//! assert_eq!(flatten::flatten_1(v), [1,2,3,4,5]);
//! ```

pub fn flatten_1 (v: Vec<Vec<isize>>) -> Vec<isize>{
  let mut vec = vec![];
  v.iter().flat_map(|i| i ).map(|i| vec.push(*i)).collect::<Vec<_>>();
  vec
}
