use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;

type Rules = HashMap<String, char>;

fn get_neighbors(pots: &str, idx: isize) -> (String, String) {
    let last = pots.len() - 1;
    let penult = last - 1;

    match idx {
        -2 => (String::from(".."), [".", &pots[0..1]].concat()),
        -1 => (String::from(".."), String::from(&pots[0..2])),
        0 => (String::from(".."), String::from(&pots[1..3])),
        1 => ([".", &pots[0..1]].concat(), String::from(&pots[2..4])),
        x if x == penult as isize => (
            String::from(&pots[((idx as usize) - 2)..idx as usize]),
            [&pots[last..], "."].concat(),
        ),
        x if x == last as isize => (
            String::from(&pots[(idx as usize - 2)..idx as usize]),
            String::from(".."),
        ),
        x if x == last as isize + 2 => ([&pots[last..], "."].concat(), String::from("..")),
        x if x == last as isize + 1 => (String::from(&pots[penult..]), String::from("..")),
        _ => (
            String::from(&pots[((idx as usize) - 2)..idx as usize]),
            String::from(&pots[((idx as usize) + 1)..((idx as usize) + 3)]),
        ),
    }
}

fn check_pot(pots: &str, idx: isize) -> char {
    if idx < 0 || idx >= pots.len() as isize {
        '.'
    } else {
        pots.chars().nth(idx as usize).unwrap()
    }
}

fn main() {
    lazy_static! {
        static ref INIT: Regex = Regex::new("initial state: ([.#]*)").expect("lol init");
        static ref RULE: Regex = Regex::new("([.#]{5}) => ([.#]{1})").expect("lol rules");
    }

    let mut rules = Rules::new();

    let input = include_str!("../input");

    let mut cur: String = INIT.captures(input.lines().next().unwrap()).unwrap()[1].to_owned();

    for line in input.lines() {
        if let Some(l) = RULE.captures(line) {
            let br = l[1].to_owned();
            match &l[2] {
                "#" => rules.insert(br, '#'),
                _ => rules.insert(br, '.'),
            };
        }
    }

    let mut leftmost: isize = 0;
    let mut old_psum: isize = 0;
    let mut delta_sum: isize = 0;
    for gen in 0.. {
        let psum: isize = cur
            .chars()
            .enumerate()
            .map(|(i, p)| if p == '#' { i as isize + leftmost } else { 0 })
            .sum();
        //println!("{}: {} {}", gen, psum, cur);
        let nd = psum - old_psum;
        if nd == delta_sum {
            println!(
                "generation {}, last dyn val: {}, delta: {}",
                gen, old_psum, nd
            );
        }
        delta_sum = nd;
        old_psum = psum;

        let clen = cur.len();
        let mut new = String::with_capacity(clen + 5);

        for i in -2..(clen + 2) as isize {
            let (lefts, rights) = get_neighbors(&cur, i);
            let pot = check_pot(&cur, i);
            let r = [lefts, pot.to_string(), rights].concat();
            if let Some(&newpot) = rules.get(&r) {
                new.push(newpot);
            }
        }

        if let Some(first_plant) = new.find('#') {
            let delta: isize = first_plant as isize - 2;
            leftmost += delta;
        }

        cur = new.trim_matches('.').to_owned();
    }
}
