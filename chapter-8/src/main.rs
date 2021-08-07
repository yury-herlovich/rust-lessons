// vectors, strings, hash maps
use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hashmaps();
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();

    println!("Vector is empty, size is {}", v.len());

    v.push(10);
    v.push(15);

    println!("few numbers were added to vector, size is {}", v.len());

    // create a vector using macro
    let mut v2 = vec![1, 5, 10];

    println!("Created a new vector, size is {}", v2.len());
    println!("Second element of vector 2 is {}", v2[1]);

    for i in &v2 {
        println!("Iterating over the vector values, el is {}", i);
    }

    for i in &mut v2 {
        let a = *i; // copy value
        *i += 100;
        println!(
            "Iterating over the vector values with changes, el {} is {}",
            a, i
        );
    }
}

fn strings() {
    let mut s = String::from("foo");
    let mut s2 = "bar";
    s.push_str(s2);
    println!("s is {}, s2 is {}", s, s2);
    s2 = "bar2";
    println!("s is {}, s2 is {}", s, s2);

    let mut s3 = String::from("abc");
    let s4 = String::from("dfe");
    s3 += &s4;

    println!("s3 is {}, s4 is {}", s3, s4);

    let s5 = format!("{}, {}, {}", s3, s4, s);
    println!("concatenated string us {}", s5);
}

fn hashmaps() {
    let mut hm = HashMap::new();

    hm.insert("blue", 10);
    hm.insert("red", 20);

    let s1 = String::from("name");
    let s2 = String::from("yury");

    let mut hm2 = HashMap::new();
    hm2.insert(s1, s2);

    println!("s1 and s2 is not available any more, ownerships were passed to hashmap");
    println!(
        "name saved in hash map is {:?}",
        hm2.get(&String::from("name"))
    );
}
