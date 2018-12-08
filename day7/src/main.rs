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

fn secs(c: &char) -> u32 {
    1 + (*c as u8 - 'A' as u8) as u32
}

fn main() {
    let r_e: Regex = Regex::new(r"Step ([A-Z]{1}) [a-z ]* ([A-Z]{1})").unwrap();

    let mut ready_at: HashMap<u32, BinaryHeap<Step>> = HashMap::new();
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
            ready_at
                .entry(60 + secs(k))
                .or_insert(BinaryHeap::new())
                .push(Step(*k));
        }
    }

    let mut t = 0;
    while !ready_at.is_empty() {
        t += 1;
        if let Some(ready) = ready_at.remove(&t) {
            for chunk in ready.iter().map(|x| *x).collect::<Vec<Step>>().chunks(5) {
                for c in chunk.iter() {
                    let c = c.0;
                    done.push(c);
                    if let Some(deps) = upstreams.remove(&c) {
                        for d in deps.iter() {
                            downstreams.entry(*d).and_modify(|x| {
                                x.remove(&c);
                            });
                            if let Some(ent) = downstreams.get(d) {
                                if ent.len() < 1 {
                                    downstreams.remove(&d);
                                    ready_at
                                        .entry(t + 60 + secs(d))
                                        .or_insert(BinaryHeap::new())
                                        .push(Step(*d));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!(
        "doing steps {} took {} seconds",
        done.iter().collect::<String>(),
        t
    );
}
