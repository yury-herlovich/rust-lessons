use std::io;

fn main() {
    vars();
    types();
    get_ar_value();
    loops();
}

/**
 * Chapter 3.1 - Variables
 */
fn vars() {
    // immutable variable
    let a = 5;
    println!("The value of a is: {}", a);

    // mutable
    let mut b = 5;
    println!("The value of b is: {}", b);

    b += 1;
    println!("New value of b is: {}", b);

    const MAX_C: u8 = 100;
    println!("The value of constant MAX_C is: {}", MAX_C);

    // shadowing variables
    let d = 1;
    let d = d + 1;
    let d = d + 2;
    println!("The value of d should be 4: d = {}", d);

    let d = "string";
    println!("We can change type of d: {}", d);
}

/**
 * Chapter 3.2 - Scalar types
 */
fn types() {
    let int: u8 = 255;
    let int2 = 255u8;
    println!(
        "int and int2 are integers with the same types, {} === {}",
        int, int2
    );

    let big_int: u64 = 1_000_000_000_000_000_000;
    println!("Big integer {}", big_int);

    let float_n = 2.0;
    println!("float_n is f64 by default, {}", float_n);

    let div = 10 / 3;
    println!("10 / 3 is {}", div);

    let float_div = 10.0 / 3.0;
    println!("10.0 / 3.0 is {}", float_div);

    let _t = true;
    let _f: bool = false; // explicit typing

    let s1 = 'w';
    let s2 = '😇';
    println!("Scalar variables - {}, {}", s1, s2);

    let t1: (char, u8, f64) = ('😇', 149, 2.4);
    println!("Tuple - {}, {}, {}", t1.0, t1.1, t1.2);

    let (a, b, c) = t1;
    println!("Tuple destruction: {}, {}, {}", a, b, c);

    let a1 = [1, 2, 3, 4];
    let _a2: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // implicit typing
    let _a3 = [3; 5]; // fill up array with the same value [3, 3, 3, 3, 3]
    println!("Array is type with fixed size = {}", a1.len());
}

fn get_ar_value() {
    let ar = ['a', 'b', 'c', 'd', 'e', 'f'];

    loop {
        println!("Please enter an array index. (0 - {})", ar.len() - 1);
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if index >= ar.len() {
            continue;
        } else {
            println!("Char: {}", ar[index]);
            break;
        }
    }
}

/**
 * Loops, while, for
 */
fn loops() {
    // endless loop
    let mut n = 0;
    loop {
        if n >= 5 {
            println!("n === {}", n);
            break;
        }

        n += 1;
    }

    // return value from loop
    let result = loop {
        if n >= 10 {
            break n;
        }
        n += 1;
    };

    println!("Loop returned new n value = {}", result);

    // while
    while n < 15 {
        n += 1;
    }

    println!("n value after while = {}", n);

    // loop through collection
    let col = [10, 20, 30, 40];
    for el in col.iter() {
        println!("el = {}", el);
    }

    // using range
    for i in 0..10 {
        println!("range: {}", i);
    }
}
