/*
Given two binary strings,
return their sum (also a binary string).
For example,
a = "11"
b = "1"
Return "100".
*/

//! # Examples
//!
//! ```
//! use algorithms::string::add_binary;
//! assert_eq!(add_binary::add_binary_1("11", "1"), "100");
//! assert_eq!(add_binary::add_binary_2("11", "1"), "100");
//! assert_eq!(add_binary::add_binary_1("11", "10"), "101");
//! assert_eq!(add_binary::add_binary_2("11", "10"), "101");
//! assert_eq!(add_binary::add_binary_1("11", "101"), "1000");
//! assert_eq!(add_binary::add_binary_2("11", "101"), "1000");

pub fn add_binary_1(a: &str, b: &str) -> String {
    let bin_a = u32::from_str_radix(a, 2).unwrap();
    let bin_b = u32::from_str_radix(b, 2).unwrap();
    let c = format!("{:b}", bin_a + bin_b);
    c
}

pub fn add_binary_2(a: &str, b: &str) -> String {
    let mut s = "".to_string();
    let mut c = 0;
    let mut i: isize = a.len() as isize - 1;
    let mut j: isize = b.len() as isize - 1;
    let zero = '0' as u8;
    let a_chars = a.chars().collect::<Vec<_>>();
    let b_chars = b.chars().collect::<Vec<_>>();

    while i >= 0 || j >= 0 || c == 1{
        if i >= 0 {
            c += a_chars[i as usize] as u8 - zero;
            println!("c: {:?}", c);
            i -= 1;
        }
        if j >= 0 {
            c += b_chars[j as usize] as u8 - zero;
            println!("c: {:?}", c);
            j -= 1;
        }
        s.insert(0, (c%2 + zero) as char );
        c /= 2;
    }
    s
}
