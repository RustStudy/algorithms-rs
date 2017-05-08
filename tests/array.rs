extern crate algorithms;
use algorithms::array::two_sum;

#[test]
fn two_sum_test() {
    let v = vec![2, 7, 11, 15];
    assert_eq!(two_sum::two_sum_1(v, 9), [0, 1]);
    let v = vec![2, 7, 11, 15];
    assert_eq!(two_sum::two_sum_2(v, 9), [0, 1]);
    let v = vec![2, 7, 11, 15];
    assert_eq!(two_sum::two_sum_3(v, 9), [0, 1]);
}
