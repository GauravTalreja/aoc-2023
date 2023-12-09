use std::collections::HashSet;

fn main() {
    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

    let mut numbers = Vec::new();
    let mut symbols = HashSet::new();
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
            if !c.is_ascii_digit() && c != '.' {
                symbols.insert((idx, lc));
            }
        }

        lc += 1;
    }

    let adjacent: HashSet<_> = symbols
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

    let sum: u64 = numbers
        .into_iter()
        .filter_map(|(k, v)| v.iter().find(|&idx| adjacent.contains(idx)).map(|_| k))
        .sum();

    println!("{sum}");
}
