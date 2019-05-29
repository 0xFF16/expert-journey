fn main() {
    let a: i32 = 10;
    let b: i32 = 20;
    let (a, b) = (b, a);
    println!("A: {}, B: {}", a, b);
}
