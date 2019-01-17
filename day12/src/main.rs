use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::str::from_utf8;

type Rules = HashMap<&'static str, bool>;

fn grow(pat: &str, rules: &Rules) -> bool {
    true
}

fn main() {
    lazy_static! {
        static ref INIT: Regex = Regex::new("initial state: ([.#]*)").expect("lol init");
        static ref RULE: Regex = Regex::new("([.#]{5}) => ([.#]{1})").expect("lol rules");
    }

    let mut input = include_str!("../input").lines();

    let ref init = INIT.captures(input.next().unwrap()).unwrap()[1];

    println!("init: {}", &init);
    let _ = input.next();

    for line in input {
        if let Some(l) = RULE.captures(line) {
            match &l[2] {
                "#" => println!("{} is YES", &l[1]),
                _ => println!("{} is NO", &l[1]),
            }
        }
    }

    for (i, w) in init.as_bytes().windows(5).enumerate() {
        println!("{}: {}", i, from_utf8(w).unwrap());
    }
}
