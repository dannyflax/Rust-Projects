fn main() {
   println!("{}", add(4,6));
}

fn add(a:i32, b:i32) -> impl<i32>{
    [a+b,a,b]
}