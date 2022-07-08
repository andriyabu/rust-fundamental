fn main() {
    //by default all variables are immutable in rust but it can be changed to mutable with "mut" keyword
    let mut x = 10;
    println!("The value of x is {}", x);

    x = 65;
    println!("The value of x is {}", x);

    // specified variables data types
    let a:i64 = 32; // i64
    let f:f32 = 5.7; // f32
    let b:bool = false; // bool

    println!("The value of a is {} integer 64", a);
    println!("The value of f is {} float 32", f);
    println!("The value of b is {} bool", b);
}
