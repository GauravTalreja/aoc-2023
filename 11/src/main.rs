use itertools::Itertools;
use std::collections::BTreeSet;

const WEIGHT: u64 = 1000000;

fn main() {
    let mut row: u64 = 0;
    let mut col: u64 = 0;
    let mut galaxies = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        col = 0;
        for c in line.chars() {
            if c == '#' {
                galaxies.push((row, col));
            }
            col += 1;
        }
        row += 1;
    }
    let rows: BTreeSet<_> = (0..row).collect();
    let galaxy_rows: BTreeSet<_> = galaxies.iter().map(|&(row, _)| row).collect();
    let weighted_rows: BTreeSet<_> = rows.difference(&galaxy_rows).cloned().collect();
    let cols: BTreeSet<_> = (0..col).collect();
    let galaxy_cols: BTreeSet<_> = galaxies.iter().map(|&(_, col)| col).collect();
    let weighted_cols: BTreeSet<_> = cols.difference(&galaxy_cols).cloned().collect();
    let mut sum = 0;
    for comb in galaxies.iter().combinations(2) {
        let &(row1, col1) = comb[0];
        let &(row2, col2) = comb[1];
        let min_row = row1.min(row2);
        let max_row = row1.max(row2);
        let min_col = col1.min(col2);
        let max_col = col1.max(col2);
        sum += max_col - min_col + max_row - min_row;
        for _ in weighted_rows
            .intersection(&(min_row..=max_row).collect())
            .chain(weighted_cols.intersection(&(min_col..=max_col).collect()))
        {
            sum += WEIGHT - 1;
        }
    }
    println!("{sum}");
}
