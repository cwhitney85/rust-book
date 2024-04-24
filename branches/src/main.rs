fn main() {
    let number: i32 = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("The value of x is: {x}");
}
