/*
Given a string, find the length of the longest substring without repeating characters.

Examples:

Given "abcabcbb", the answer is "abc", which the length is 3.

Given "bbbbb", the answer is "b", with the length of 1.

Given "pwwkew", the answer is "wke", with the length of 3.
Note that the answer must be a substring,
"pwke" is a subsequence and not a substring.

*/

//! # Examples
//!
//! ```
//! use algorithms::array::longest_non_repeat;
//! let s = String::from("abcabcbb");
//! assert_eq!(longest_non_repeat::longest_non_repeat_1(s), 3);
//! let s = String::from("abcabcdefbb");
//! assert_eq!(longest_non_repeat::longest_non_repeat_1(s), 6);
//! let s = String::from("bbbbb");
//! assert_eq!(longest_non_repeat::longest_non_repeat_1(s), 1);
//! let s = String::from("pwwkew");
//! assert_eq!(longest_non_repeat::longest_non_repeat_1(s), 3);

//! assert_eq!(longest_non_repeat::longest_non_repeat_2("abcabcbb"), 3);

//! ```

use std::collections::HashMap;

pub fn longest_non_repeat_1(str: String) -> usize {
    // 用于记录上一个重复字母起始位置而非数组索引，从1开始记录
    let mut last_start = 0;
    // 用于记录不重复子串的最大长度
    let mut maxlen = 0;
    // 用于排查统计过的字母在数组中的索引：{'c': 2, 'b': 1, 'a': 0}
    let mut used_char: HashMap<char, usize>  = HashMap::new();

    let letters = str.chars().collect::<Vec<_>>();
    for i in 0..letters.len() {
        if used_char.contains_key(&letters[i]) && last_start <=used_char[&letters[i]]{
            // 如果之前已经统计过该字母，则更新其最新的位置
            last_start = used_char[&letters[i]] + 1;
        }else {
            // 对没有重复出现过的字母进行长度统计
            // i - last_start + 1是去除之前的重复子串长度
            maxlen =  if maxlen > (i - last_start + 1) { maxlen } else { i - last_start + 1 };
        }
        used_char.insert(letters[i], i);
    }
    maxlen
}

pub fn longest_non_repeat_2(str: &str)  -> usize {
    let mut last_start = 0;
    let mut maxlen = 0;
    let mut used_char: HashMap<char, usize>  = HashMap::new();

    let letters = str.chars().collect::<Vec<_>>();
    for i in 0..letters.len() {
        if used_char.contains_key(&letters[i]) && last_start <=used_char[&letters[i]]{
            last_start = used_char[&letters[i]] + 1;
        }else {
            maxlen =  if maxlen > (i - last_start + 1) { maxlen } else { i - last_start + 1 };
        }
        used_char.insert(letters[i], i);
    }
    maxlen

}
