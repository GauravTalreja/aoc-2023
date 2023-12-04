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
            if !game
                .sets
                .iter()
                .any(|set| set.red > RED_COUNT || set.blue > BLUE_COUNT || set.green > GREEN_COUNT)
            {
                sum += game.id;
            }
        });

    println!("{sum}")
}
