fn main() {
    //by default all variables are immutable in rust but it can be changed to mutable with "mut" keyword
    let mut x = 10;
    println!("The value of x is {}", x);

    x = 65;
    println!("The value of x is {}", x);
}
