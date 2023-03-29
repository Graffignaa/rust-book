fn main() {
    let gifts = [
        "A Partridge In A Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for i in 1..13 {
        prefix(i);
        for j in (0..i).rev() {
            if j == 0 && i != 1 {
                print!("And ");
            }
            println!("{}", gifts[j]);
        }
        println!("");
    }
}

fn prefix(n: usize) {
    let ord_suffix = match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    println!("On the {n}{ord_suffix} day of Christmas my true love gave to me: ");
}
