macro_rules! hash(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn main() {
    let names = hash!{ 1 => "one", 2 => "two" };
    println!("{:?}",  names);
    println!("{} -> {:?}", 1, names.get(&1));
    println!("{} -> {:?}", 10, names.get(&10));
}
