use std::cmp::PartialOrd;

fn main() {
    find_largest();
    find_longest();
}

fn find_longest() {
    let a = "abcd";
    let b = "abcdef";

    println!("the longest string is {}", longest(a, b))
}

// 'a - lifetime annotation, requires that both parameters must both live as long as that generic lifetime
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn find_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
