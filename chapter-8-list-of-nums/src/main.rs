use chapter_8_list_of_nums::utils;
use std::collections::HashMap;

fn main() {
    println!("Please enter a number between 1 and 20");

    let mut num: u32;

    loop {
        let input = utils::wait_for_input();
        num = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again");
                continue;
            }
        };

        if num < 1 && num > 20 {
            println!("Try again");
            continue;
        }

        break;
    }

    let v = utils::gen_nums(num);

    println!("Created a list of random numbers: {:?}", v);
    println!("Average value is {}", average(&v));
    println!("Median value is {}", median(&v));
    println!("Mode value is {}", mode(&v));
}

fn average(v: &Vec<u16>) -> u16 {
    let mut sum = 0;

    for item in v {
        sum += item;
    }

    sum / (v.len() as u16)
}

fn median(v: &Vec<u16>) -> u16 {
    let mut new_vec = v.to_vec();
    new_vec.sort();

    v[new_vec.len() / 2]
}

fn mode(v: &Vec<u16>) -> u16 {
    let mut items: HashMap<u16, u16> = HashMap::new();

    for item in v {
        let i = items.entry(*item).or_insert(0);
        *i += 1;
    }

    let mut result = 0;
    let mut max = 0;

    for (k, v) in items {
        if v > max {
            max = v;
            result = k;
        }
    }

    result
}
