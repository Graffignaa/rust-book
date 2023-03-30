fn main() {
    let s1 = gives_ownership(); // Gives ownership and moves return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into function, which also moves its
                                       // return value into s3
                                       //
                                       //
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

}

fn gives_ownership() -> String {
    // function will move its return value into
    // the function that calls it
    let some_string = String::from("yours");

    some_string
}

// Takes a string and returns one
fn takes_and_gives_back(a_string: String) -> String {
    //a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); 
    (s, length)
}
