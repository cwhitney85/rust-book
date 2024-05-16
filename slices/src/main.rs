fn main() {
    let my_string = String::from("hello world");

    // let word = first_word(&my_string);
    // let word = first_word(&my_string[..]);
    let word = first_word(&my_string[..6]);

    println!("The first word is {}", word);

    let my_string_literal = "hello world";

    // let word = first_word(&my_string_literal[..6]);
    // let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);

    println!("The first word of my_string_literal is {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
