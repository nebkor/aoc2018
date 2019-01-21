use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;

type Rules = HashMap<String, char>;

fn beginning(cur: &str, new: &mut String, rules: &Rules, leftest: &mut isize, idx: usize) {
    if idx != 0 {
        return;
    }
    let mut nleft: isize = 0;
    for neg in 1..=2 {
        let r = match neg {
            1 => ["...", &cur[0..2]].concat().to_owned(),
            2 => ["....", &cur[0..1]].concat().to_owned(),
            _ => unreachable!(),
        };

        if let Some(&p) = rules.get(&r) {
            new.insert(0, p);
            if p == '#' {
                nleft = neg as isize;
            }
        }
    }

    let mut first_plant = -1;

    for pot in 0..2 {
        let r = match pot {
            0 => ["..", &cur[0..3]].concat().to_owned(),
            1 => [".", &cur[0..4]].concat().to_owned(),
            _ => unreachable!(),
        };

        if let Some(&p) = rules.get(&r) {
            new.push(p);
            if p == '#' {
                first_plant = pot;
            }
            if nleft == 0 && p == '.' && first_plant < 0 {
                *leftest += 1;
            }
        }
    }

    *leftest -= nleft;

    dbg!(&new);

    //dbg!(leftest);
}

fn ending(cur: &str, new: &mut String, rules: &Rules, idx: usize) {
    if idx != cur.len() - 2 {
        return;
    }

    for i in 0..4 {
        let r = match i {
            0 => [&cur[(idx - 2)..(idx + 2)], "."].concat().to_owned(),
            1 => [&cur[(idx - 1)..(idx + 2)], ".."].concat().to_owned(),
            2 => [&cur[idx..(idx + 2)], "..."].concat().to_owned(),
            3 => [&cur[(idx + 1)..], "...."].concat().to_owned(),
            _ => return,
        };

        if let Some(&pot) = rules.get(&r) {
            new.push(pot);
            println!("{}: {}, {}\n{}", i, &r, pot, cur);
        }
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
    for _ in 0..20 {
        let clen = cur.len();
        let mut new = String::with_capacity(clen + 5);
        for (i, _p) in cur.chars().enumerate() {
            if i < 2 {
                beginning(&cur, &mut new, &rules, &mut leftmost, i);
                continue;
            }

            if i < clen - 2 {
                // we're in the middle now
                let r = &cur[(i - 2)..(i + 3)];

                if let Some(&pot) = rules.get(r) {
                    new.push(pot);
                }
            } else {
                ending(&cur, &mut new, &rules, i);
            }
        }

        cur = new.trim_matches('.').to_owned();
        //println!("{}{}", vec![" "; (12 + leftmost) as usize].concat(), cur);
    }

    let part_1_answer: isize = cur
        .chars()
        .enumerate()
        .map(|(i, b)| if b == '#' { i as isize + leftmost } else { 0 })
        .sum();

    println!("Part 1: {}", part_1_answer);
}
