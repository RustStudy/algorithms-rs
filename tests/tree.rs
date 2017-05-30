extern crate algorithms;
use algorithms::tree::binary_tree::{BinTree};

#[test]
fn binary_tree_test() {
    let mut tree = BinTree::new();
    tree.insert(2);
    tree.insert(0);
    tree.insert(3);
    tree.insert(1);
    tree.insert(4);
    println!("{:#?}", tree);
    println!("{}", tree);
}
