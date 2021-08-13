use chapter_8_pig_latin::utils;
use regex::Regex;

fn main() {
    // enter a phrase or use default
    println!("Please enter a phrase or click enter to use default");

    let mut phrase = String::from("Hello, world!");
    loop {
        let input = utils::wait_for_input();
        phrase = match input.trim().parse::<String>() {
            Ok(in_phrase) => {
                if in_phrase.chars().count() == 0 {
                    println!("Using default {}", phrase);
                    break;
                }

                in_phrase
            }
            Err(_) => {
                println!("Try again");
                continue;
            }
        };

        break;
    }

    println!("This phrase in pig latin: {}", to_pig_latin(phrase));
}

fn to_pig_latin(str: String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut v: Vec<String> = Vec::new();

    for word in split_by_words(&str) {
        let mut symbols = word.chars();
        let first_symbol = symbols.nth(0).unwrap();

        if vowels.contains(&first_symbol) {
            v.push(format!("{}-{}", word, "hay"))
        } else {
            v.push(format!(
                "{}-{}{}",
                symbols.collect::<String>(),
                first_symbol,
                "ay"
            ))
        }
    }

    v.join(" ").to_lowercase()
}

fn split_by_words(phrase: &String) -> Vec<String> {
    // TODO: leave punctuation
    let re = Regex::new(r"[^a-zA-Z]").unwrap();
    let mut v: Vec<String> = re
        .replace_all(phrase, " ")
        .split(" ")
        .map(String::from)
        .collect();

    // remove empty words
    v.retain(|word| word.chars().count() > 0);

    return v;
}
