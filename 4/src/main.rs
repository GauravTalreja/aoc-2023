use std::collections::HashSet;

fn main() {
    let mut sum: u32 = 0;

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
            let score = line
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .fold(0, |mut acc, have| {
                    if win.contains(&have) {
                        if acc == 0 {
                            acc += 1;
                        } else {
                            acc *= 2;
                        }
                    }
                    acc
                });
            sum += score
        }
    });

    println!("{sum}")
}
