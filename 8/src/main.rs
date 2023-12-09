use num::integer::lcm;
use std::collections::HashMap;

const START: char = 'A';
const END: char = 'Z';

fn main() {
    let mut lines = std::io::stdin().lines();
    let instructions = lines.next().unwrap().unwrap();
    lines.next();
    let map: HashMap<_, _> = lines
        .map(|line| {
            let mut line = line.as_ref().unwrap().split('=');
            let key = line.next().unwrap()[..3].to_owned();
            let mut values = line.next().unwrap().split(",");
            let left = values.next().unwrap()[2..].to_owned();
            let right = values.next().unwrap()[1..4].to_owned();
            (key, (left, right))
        })
        .collect();

    let elements: Vec<_> = map
        .clone()
        .into_keys()
        .filter(|key| key.chars().nth(2) == Some(START))
        .collect();

    let steps = elements.iter().map(|element| {
        let mut element = element.to_owned();
        let mut count: usize = 0;
        let mut chars = instructions.chars();
        while element.chars().nth(2) != Some(END) {
            count += 1;

            let mut dir = chars.next();
            if dir.is_none() {
                chars = instructions.chars();
                dir = chars.next();
            }

            let dir = dir.unwrap();
            let (left, right) = map.get(&element).unwrap();
            match dir {
                'L' => {
                    element = left.to_owned();
                }
                'R' => {
                    element = right.to_owned();
                }
                _ => unreachable!(),
            }
        }
        count
    });

    let count = steps.fold(1, |acc, step| lcm(acc, step));
    println!("{count}");
}
