fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (_a, b, _c) = tup;

    println!("The value of b is: {b}");

    let i = tup.0;
    println!("The value of the first element is {i}");
}