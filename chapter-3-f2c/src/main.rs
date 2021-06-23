use std::io;

fn main() {
    loop {
        println!("Enter temperature in Fahrenheit or 'q' to quit...");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            break;
        }

        let input: i16 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input, please try again");
                continue;
            }
        };

        let temp_c = calculate(input);

        println!("{}°F equivalent to {}°C", input, temp_c);
    }
}

/**
 * Converts Fahrenheit to Celsius
 */
fn calculate(temp: i16) -> i16 {
    let c = (temp - 32) as f64 / 1.8;
    return c.round() as i16;
}
