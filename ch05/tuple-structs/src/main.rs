// These are different types, even though they have they have the same fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-Like Struct
struct AlwaysEqual;


fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 1, 2);

    // Tuple structs can be indexed into with a . followed by the index
    println!("{} {} {}", black.0, black.1, black.2);

    println!("{} {} {}", origin.0, origin.1, origin.2);

    let subject = AlwaysEqual; // Imagine we later implement this to always be equal to any
                               // other instance of any type.  We wouldnt need any data 
                               // to do this, so we use a unit-like struct.

}
