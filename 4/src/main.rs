use std::collections::{HashMap, HashSet};

fn main() {
    let mut counts: HashMap<u32, usize> = Default::default();

    std::io::stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut line = line.split(':');
            let id: u32 = line
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let mut line = line.next().unwrap().split("|");
            let win = line
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let wins = line
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .filter(|have| win.contains(have))
                .count();

            if let Some(count) = counts.get_mut(&id) {
                *count += 1;
            } else {
                counts.insert(id, 1);
            };

            let count = *counts.get_mut(&id).unwrap();

            for i in 1..=wins {
                let id = id + i as u32;
                if let Some(next) = counts.get_mut(&id) {
                    *next += count;
                } else {
                    counts.insert(id, count);
                };
            }
        }
    });

    let sum: usize = counts.into_values().sum();

    println!("{sum}")
}
