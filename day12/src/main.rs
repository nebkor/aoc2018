use lazy_static::lazy_static;
use regex::Regex;

use std::collections::{HashMap, VecDeque};

type Rules = HashMap<Vec<bool>, bool>;
type Pots = VecDeque<bool>;

fn beginning(cur: &Pots, idx: usize, rules: &Rules, new: &mut Pots) -> isize {
    // only act on first call, and handle up to 2
    if idx != 0 {
        return 0;
    }

    let mut negs: isize = 0;

    for neg in 1..=3 {
        let p: Vec<bool> = vec![false; 4 - neg]
            .iter()
            .chain(cur.iter().take(3 - neg))
            .map(|b| *b)
            .collect();
        if let Some(&pot) = rules.get(&p) {
            if pot {
                new.push_front(true);
                negs += 1;
            }
        }
    }

    for pos in 0..=1 {
        let p: Vec<bool> = vec![false; 2 - pos]
            .iter()
            .chain(cur.iter().skip(pos).take(3 - pos))
            .map(|b| *b)
            .collect();
        if let Some(&pot) = rules.get(&p) {
            new.push_back(pot);
        } else {
            new.push_back(cur[pos]);
        }
    }

    negs
}

fn ending(cur: &Pots, start_from: usize, rules: &Rules, new: &mut Pots) {
    let mut last_pot_checked = start_from + 3;
    let clen = cur.len();
    while last_pot_checked < clen {
        let leftc = cur
            .iter()
            .skip(last_pot_checked - 2)
            .take(3)
            .map(|b| *b)
            .collect::<Vec<bool>>();
        let rightmost = last_pot_checked + 2;
        if rightmost >= clen {
            let fill = (rightmost - clen) + 1;
            let pat = leftc
                .iter()
                .chain(vec![false; fill].iter())
                .map(|b| *b)
                .collect::<Vec<bool>>();
            if let Some(pot) = rules.get(&pat) {
                new.push_back(*pot);
            } else {
                new.push_back(cur[last_pot_checked]);
            }
        } else {
            let pat = leftc
                .iter()
                .chain(cur.iter().skip(last_pot_checked + 1).take(2))
                .map(|b| *b)
                .collect::<Vec<bool>>();
            if let Some(pot) = rules.get(&pat) {
                new.push_back(*pot);
            } else {
                new.push_back(cur[last_pot_checked])
            }
        }
        last_pot_checked += 1;
    }
    let left = cur
        .iter()
        .skip(last_pot_checked - 1)
        .take(2)
        .map(|b| *b)
        .collect::<Vec<bool>>();
    let cright = vec![false; 3];

    let pat = left
        .iter()
        .chain(cright.iter())
        .map(|b| *b)
        .collect::<Vec<bool>>();

    if let Some(pot) = rules.get(&pat) {
        if *pot {
            new.push_back(true);
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

    let mut cur: Pots = INIT.captures(input.lines().next().unwrap()).unwrap()[1]
        .chars()
        .map(|c| c == '#')
        .collect();

    for line in input.lines() {
        if let Some(l) = RULE.captures(line) {
            let br: Vec<bool> = l[1].chars().map(|p| p == '#').collect();
            match &l[2] {
                "#" => rules.insert(br, true),
                _ => rules.insert(br, false),
            };
        }
    }

    let mut tot_negs: isize = 0;
    for _ in 0..20 {
        let mut negs: isize = 0;
        let mut end: usize = 0;
        let mut new: Pots = Pots::with_capacity(cur.len() + 5);
        for (i, w) in cur
            .iter()
            .map(|b| *b)
            .collect::<Vec<bool>>()
            .windows(5)
            .enumerate()
        {
            end = i;
            if i < 2 {
                negs += beginning(&cur, i, &rules, &mut new);
            }

            // we're in the middle now
            if let Some(pot) = rules.get(w) {
                new.push_back(*pot);
            } else {
                new.push_back(cur[i + 2]);
            }
        }
        tot_negs += negs;

        ending(&cur, end, &rules, &mut new);

        println!(
            "{}{}",
            vec![" "; (40 - negs) as usize].join(""),
            cur.iter()
                .map(|c| if *c { "#" } else { "." })
                .collect::<Vec<_>>()
                .join("")
        );

        cur = new;
    }

    let part_1_answer: isize = cur
        .iter()
        .enumerate()
        .map(|(i, b)| if *b { i as isize - tot_negs } else { 0 })
        .sum();

    println!("Part 1: {}", part_1_answer);
}
