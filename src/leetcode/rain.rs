/*
leetcode 42:

Given [0,1,0,2,1,0,1,3,2,1,2,1], return 6.

 ^
3|              ■           □: water
2|      ■ □ □ □ ■ ■ □ ■     ■: elevation map
1|  ■ □ ■ ■ □ ■ ■ ■ ■ ■ ■
  ————————————————————————>
The above elevation map is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.

给定一个数组，数组种的数字代表方块的高度，问能盛多少水，上图种空心方块表示水，实心方块表示方块

思路：

1. 左右两个指针pl和pr，分别向中间扫描
2. 记录左右两边当前最高的方块maxl和maxr
3. 如果maxl小于等于maxr，移动pl，计算maxl和水
4. 如果maxl大于maxr，移动pr，计算maxr和水
5. pl和pr相遇的时候结束
*/

//! # Examples
//!
//! ```
//! use algorithms::leetcode::rain;
//! let v = vec![0,1,0,2,1,0,1,3,2,1,2,1];
//! assert_eq!(rain::rain(v), 6);
//! ```


pub fn rain(vec: Vec<usize>) -> usize {
    let (mut water_num, mut maxl, mut maxr, mut pl, mut pr) = (0, 0, 0, 0, vec.len() - 1);

    while pl <= pr {
        if maxl <= maxr{
            if maxl < vec[pl] {
                maxl = vec[pl];
            }else{
                water_num += maxl-vec[pl];
            }
            pl += 1;
        } else {
            if maxr < vec[pr]{
                maxr = vec[pr];
            }else{
                water_num += maxr-vec[pr];
            }
            pr-=1;
        }
    }

    water_num
}
