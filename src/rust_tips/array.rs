fn change(arr: &mut [[u32; 2]]) {
    arr.sort();
    arr[0] = [arr[0][0], arr[1][1]];

}

fn main() {

   let mut x = [[2,6],[1,3],[8,10],[15,18]];
   change(&mut x);

   print!("{:?}", x);

}
