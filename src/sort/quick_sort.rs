/*
Quicksort: Complexity: best O(n) avg O(n log(n)), worst O(N^2)
*/

//! # Examples
//!
//! ```
//! use algorithms::sort::quick_sort;
//! let mut v = vec![1,5,65,23,57,1232,-1,-5,-2,242,100,4,423,2,564,9,0,10,43,64];
//! let last = v.len() - 1;
//! quick_sort::quick_sort_1(&mut v, 0, last);
//! assert_eq!(v, [-5, -2, -1, 0, 1, 2, 4, 5, 9, 10, 23, 43, 57, 64, 65, 100, 242, 423, 564, 1232]);
//! ```

pub fn quick_sort_1(vec: &mut Vec<isize>, first: usize, last: usize) {
    if first < last {
        let pos = partition(vec, first, last);
        quick_sort_1(vec, first, pos-1);
        quick_sort_1(vec, pos+1, last);
    }
}

fn partition(vec: &mut Vec<isize>, first: usize, last: usize) -> usize {
    let mut wall = first;
    for pos in first..last {
        if vec[pos] < vec[last] {
            vec.swap(pos, wall);
            wall += 1;
        }
    }
    vec.swap(wall, last);
    wall
}
