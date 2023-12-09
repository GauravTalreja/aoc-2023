use std::collections::{HashMap, HashSet};

fn main() {
    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

    let mut numbers = Vec::new();
    let mut gears = HashSet::new();
    let mut lc = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        let mut acc = String::new();
        let mut start = 0;
        for (idx, digit) in line.match_indices(digits) {
            if acc.is_empty() {
                start = idx;
            }

            if !acc.is_empty() && idx != start + acc.len() {
                let number: u64 = acc.parse().unwrap();
                numbers.push((
                    number,
                    (start..start + acc.len())
                        .map(|i| (i, lc))
                        .collect::<Vec<_>>(),
                ));
                acc.clear();
                start = idx;
            }
            acc += digit;
        }
        if !acc.is_empty() {
            let number: u64 = acc.parse().unwrap();
            numbers.push((
                number,
                (start..start + acc.len())
                    .map(|i| (i, lc))
                    .collect::<Vec<_>>(),
            ));
        }

        for (idx, c) in line.char_indices() {
            if c == '*' {
                gears.insert((idx, lc));
            }
        }

        lc += 1;
    }

    let adjacent: HashSet<_> = gears
        .iter()
        .flat_map(|&(i, lc)| {
            vec![
                (i - 1, lc - 1),
                (i + 1, lc + 1),
                (i - 1, lc + 1),
                (i + 1, lc - 1),
                (i, lc + 1),
                (i, lc - 1),
                (i + 1, lc),
                (i - 1, lc),
            ]
        })
        .collect();

    let adjacent: HashMap<_, _> = numbers
        .into_iter()
        .filter_map(|(k, v)| {
            v.iter()
                .find(|&idx| adjacent.contains(idx))
                .map(|&idx| (idx, k))
        })
        .collect();

    let sum: u64 = gears
        .iter()
        .map(|&(i, lc)| {
            let filter = vec![
                (i - 1, lc - 1),
                (i + 1, lc + 1),
                (i - 1, lc + 1),
                (i + 1, lc - 1),
                (i, lc + 1),
                (i, lc - 1),
                (i + 1, lc),
                (i - 1, lc),
            ];
            let filter: Vec<_> = filter
                .iter()
                .filter(|idx| adjacent.contains_key(idx))
                .collect();
            if filter.len() == 2 {
                filter
                    .iter()
                    .map(|idx| adjacent.get(idx).unwrap())
                    .product()
            } else {
                0
            }
        })
        .sum();

    println!("{sum}");
}
