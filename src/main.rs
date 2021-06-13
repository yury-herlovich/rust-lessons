use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let out = b"Hello fellow!";
    let width = 12;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();

    println!("Hello from println!")
}
