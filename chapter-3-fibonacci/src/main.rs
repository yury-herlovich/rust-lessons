use std::io;

fn main() {
    loop {
        println!("Enter number or 'q' to quit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            break;
        }

        let input: u16 = match input.trim().parse() {
            Ok(num) => {
                if num > 185 {
                    println!("Too big, please try again");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Wrong input, please try again");
                continue;
            }
        };

        let num = calculate(input);

        println!("{} fibonacci number is {}", input, num)
    }
}

fn calculate(n: u16) -> u128 {
    if n < 2 {
        return n as u128;
    }

    let mut a = 0;
    let mut b = 1;
    for _i in 2..n {
        let temp = a;
        a = b;
        b += temp;
    }

    return a + b;
}
