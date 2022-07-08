fn main() {
    let mut x = 0;

    loop{
        x += 1;
        if x == 7{
            continue;
        }

        if x > 10{
            break;
        }

        println!("number {}", x);
    }
}
