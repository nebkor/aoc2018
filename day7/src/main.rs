use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Step(char);

impl Ord for Step {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0).reverse()
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&rhs.0).reverse())
    }
}

fn main() {
    let r_e: Regex = Regex::new(r"Step ([A-Z]{1}) [a-z ]* ([A-Z]{1})").unwrap();

    let mut todo: BinaryHeap<Step> = BinaryHeap::new();
    let mut done: Vec<char> = Vec::with_capacity(26);
    let mut upstreams: HashMap<char, HashSet<char>> = HashMap::new();
    let mut downstreams: HashMap<char, HashSet<char>> = HashMap::new();

    for line in include_str!("../input").lines() {
        let mut pre_req: char = '_';
        let mut demand: char = '_';
        for c in r_e.captures(line).unwrap()[1].chars() {
            pre_req = c;
        }
        for c in r_e.captures(line).unwrap()[2].chars() {
            demand = c;
        }
        upstreams
            .entry(pre_req)
            .or_insert(HashSet::new())
            .insert(demand);

        downstreams
            .entry(demand)
            .or_insert(HashSet::new())
            .insert(pre_req);
    }

    for k in upstreams.keys() {
        if !downstreams.contains_key(k) {
            todo.push(Step(*k));
        }
    }

    while !todo.is_empty() {
        let cur = todo.pop().unwrap().0;
        done.push(cur);

        if let Some(deps) = upstreams.remove(&cur) {
            for d in deps.iter() {
                downstreams.entry(*d).and_modify(|x| {
                    x.remove(&cur);
                });
                if let Some(ent) = downstreams.get(d) {
                    if ent.len() < 1 {
                        downstreams.remove(&d);
                        todo.push(Step(*d));
                    }
                }
            }
        }
    }

    println!("{}", done.iter().collect::<String>());
}
