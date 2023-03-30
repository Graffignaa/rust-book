fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    // Ints are stored on the stack, known size at compile time, quick to copy
    // No reason to prevent x from being valid after creating y, so rust doesnt invalidate it
    // No difference between deep and shallow copying
    // clone wouldnt do anything different, so we can omit
    //
    // This works because ints implement the Copy trait
    // Copy trait can't be implemented if a type or any of its parts implement the Drop trait
    // String, for example, implements drop

    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);
    //s1 is invalid here
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function, no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is copy. ok to use x afterward
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
