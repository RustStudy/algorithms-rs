/*
Given a collection of intervals, merge all overlapping intervals.
For example,
Given [1,3],[2,6],[8,10],[15,18],
return [1,6],[8,10],[15,18].


1___2___3
    2___3___4___5___6
有重复，所以合并为1___2___3___4___5___6

注意：子数组是乱序的

思路：

1. 按左边界排序
2. 合并重复
3. 输出

*/

//! # Examples
//!
//! ```
//! use algorithms::array::merge_intervals;
//! let mut x = vec![[2,6],[1,3],[8,10],[15,18]];
//! merge_intervals::merge_intervals_1(&mut x);
//! assert_eq!(x, [[1, 6], [8, 10], [15, 18]]);
//!
//! let mut y = vec![[3,6],[1,5],[7,11],[9,12], [4, 8]];
//! merge_intervals::merge_intervals_1(&mut y);
//! assert_eq!(y, [[1, 12]]);

pub fn merge_intervals_1(arr: &mut Vec<[u32; 2]>) {
    arr.sort();
    for i in 0..arr.len()-1 {
        if arr[i][1] > arr[i+1][0] {
            arr[i+1] = [arr[i][0], arr[i+1][1]];
            arr[i] = [0,0]; //填充[0, 0]，避免数组长度发生变化
        }
    }
   // 统一去除[0,0]
   for i in 0..arr.len()-1 {
       let index = arr.iter().position(|x| *x == [0,0]);
       match index {
           Some(_) => arr.remove(index.unwrap()),
           _ => arr[i],
       };
   }
}
