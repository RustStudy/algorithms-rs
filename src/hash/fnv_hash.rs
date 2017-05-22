/*
OFFSET_BASIS = {
      32 => 0x811c9dc5,
      64 => 0xcbf29ce484222325,
      128 => 0x6c62272e07bb014262b821756295c58d
    }

PRIME = {
  32 => 16777619,
  64 => 1099511628211,
  128 => 309485009821345068724781371
}

MASK = {
  32 => 4294967295,
  64 => 18446744073709551615,
  128 => 340282366920938463463374607431768211455
}


*/
//! # Examples
//!
//! ```
//! use algorithms::hash::fnv_hash;
//! let nums = vec![1,2,3,4,5,6,7,8,9];
//! assert_eq!(fnv_hash::fnv_1(b"hello"), [3, 6, 9, 4, 8, 5, 2, 7, 1]);
//! ```


// 64 size
// pub fn fnv_1(bytes: &[u8]) -> u64 {
//     let offset_basis = 0xcbf29ce484222325;
//     let prime = 1099511628211;
//     let mut hash = offset_basis;
//     for byte in bytes.iter() {
//         hash *= prime;
//         hash &= 18446744073709551615;
//         hash ^= *byte as u64;
//     }
//     hash
// }
