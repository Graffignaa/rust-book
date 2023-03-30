fn main() {
    let mut s = String::from("hello world");

    // Clunky
    let word = bad_first_word(&s);

    s.clear();

    // Use slices instead

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11]; // Manually taking slices

    let word = first_word(&s);

    println!("the first word is: {}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
// Clunky, need to manage indices, even worse for second, nth word.
fn bad_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
// better, with slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // Equivalent to &s[0..i]
        }
    }

    &s[..] // Equivalent to &s[0..0]
}
