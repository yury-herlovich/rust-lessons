fn main() {
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
