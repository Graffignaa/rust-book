use std::io;
fn main() {
    let n = loop {
        println!("Enter a positive integer: ");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break n;
    };

    let fibn = fib(n);

    if n == 1 {
        println!("The {n}st fibbonacci number is {fibn}");
    } else if n == 2 {
        println!("The {n}nd fibbonacci number is {fibn}");
    } else if n == 3 {
        println!("The {n}rd fibbonacci number is {fibn}");
    } else {
        println!("The {n}th fibbonacci number is {fibn}");
    }
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
