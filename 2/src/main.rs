const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

const RED_COUNT: u32 = 12;
const GREEN_COUNT: u32 = 13;
const BLUE_COUNT: u32 = 14;

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn main() {
    let mut sum: u32 = 0;

    std::io::stdin()
        .lines()
        .filter_map(|line| {
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
                let line = line.next().unwrap();
                let sets = line
                    .split(';')
                    .map(|game| {
                        let mut set = Set {
                            red: 0,
                            green: 0,
                            blue: 0,
                        };
                        game.split(',').for_each(|cube| {
                            let mut cube = cube.split_whitespace();
                            let count: u32 = cube.next().unwrap().parse().unwrap();
                            let color = cube.next().unwrap();
                            match color {
                                RED => {
                                    set.red = count;
                                }
                                GREEN => {
                                    set.green = count;
                                }
                                BLUE => {
                                    set.blue = count;
                                }
                                _ => panic!(),
                            }
                        });
                        set
                    })
                    .collect::<Vec<Set>>();
                return Some(Game { id, sets });
            }
            None
        })
        .for_each(|game| {
            let min = game.sets.iter().fold(
                Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, set| {
                    if set.red > acc.red {
                        acc.red = set.red;
                    }
                    if set.green > acc.green {
                        acc.green = set.green;
                    }
                    if set.blue > acc.blue {
                        acc.blue = set.blue;
                    }
                    acc
                },
            );
            let power = min.red * min.green * min.blue;
            sum += power;
        });

    println!("{sum}")
}
