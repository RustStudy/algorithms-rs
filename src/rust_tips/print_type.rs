#![feature(core_intrinsics)]

fn print_type_of<T>(_: &T) {
    println!("{:?}", unsafe { std::intrinsics::type_name::<T>() });
}


// rustc --out-dir target/ src/rust_tips/print_type.rs

fn main(){
    print_type_of(&32.90);          // prints "f64"
    print_type_of(&vec![1, 2, 4]);  // prints "std::vec::Vec<i32>"
    print_type_of(&"foo");          // prints "&str"
    println!("{:?}", 'h');
    /*
    make use of rustc error:

    13 |     let mut my_number: () = 32.90; // rustc print type error
       |                             ^^^^^ expected (), found floating-point variable
       |
       = note: expected type `()`
              found type `{float}`
    */

    // let mut my_number: () = 32.90;
}
