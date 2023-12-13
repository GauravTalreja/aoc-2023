fn main() {
    let sum: i64 = std::io::stdin()
        .lines()
        .map(|line| {
            let sequence: Vec<i64> = line
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let mut sequences = vec![sequence];
            while sequences.last().unwrap().iter().any(|&v| v != 0) {
                let sequence = sequences.last().unwrap();
                let diffs: Vec<i64> = sequence[..sequence.len() - 1]
                    .iter()
                    .zip(sequence[1..].iter())
                    .map(|(&a, &b)| b - a)
                    .collect();
                sequences.push(diffs);
            }
            sequences
                .iter()
                .rfold(0, |acc, seq: &Vec<i64>| seq[0] - acc)
        })
        .sum();
    println!("{sum}");
}
