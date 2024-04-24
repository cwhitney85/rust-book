fn five() -> i32 {
    5
}

fn main() {
    another_function(5);

    print_labeled_measurements(5, 'h');

    let a = five();

    println!("The value of a is: {a}");
    let i = plus_one(a);
    println!("The value of i is: {i}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}.")
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(y: i32) -> i32 {
    y + 1
}