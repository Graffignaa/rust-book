use std::io;
fn main() {
    let temp = loop {
        println!("Enter a temperature to convert: ");
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break temp;
    };

    //println!("temp: {temp}");

    let scale = loop {
        println!("Enter the scale (F for Farenheit, C for Celsius)");

        let mut scale = String::new();

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");
        let scale: char = match scale.trim().to_ascii_uppercase().parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        if scale == 'F' || scale == 'C' {
            break scale;
        }
    };

    let (temp, scale) = convert_temp(temp, scale);
    println!("The converted temperature is {temp}°{scale}");
}

fn convert_temp(temp: f64, scale: char) -> (f64, char) {
    if scale == 'F' {
        return ((temp - 32.0) * (5.0 / 9.0), 'C');
    } else if scale == 'C' {
        return ((temp + 32.0) * (9.0 / 5.0), 'F');
    } else {
        return (-1.0, 'U');
    }
}
