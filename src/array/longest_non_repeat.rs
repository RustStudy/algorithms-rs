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
//! ```

use std::collections::HashMap;

pub fn longest_non_repeat_1(str: String) -> usize {
    let mut start = 0;
    let mut maxlen = 0;
    let mut used_char: HashMap<char, usize>  = HashMap::new();
    let letters = str.chars().collect::<Vec<_>>();

    for i in 0..letters.len() {
        if used_char.contains_key(&letters[i]) && start <=used_char[&letters[i]]{
            start = used_char[&letters[i]] + 1;
        }else {
            maxlen =  if maxlen > (i - start + 1) { maxlen } else { i - start + 1 };
        }
        used_char.insert(letters[i], i);
    }
    maxlen
}
