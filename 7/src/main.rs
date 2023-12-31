#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn from_char(c: char) -> Option<Card> {
        match c {
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            'T' => Some(Self::Ten),
            'J' => Some(Self::Joker),
            'Q' => Some(Self::Queen),
            'K' => Some(Self::King),
            'A' => Some(Self::Ace),
            _ => None,
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Hand {
    pub cards: [Card; 5],
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Rank {
    High,
    One,
    Two,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct HandRank {
    pub rank: Rank,
    pub hand: Hand,
}

impl HandRank {
    pub fn from_str(str: &str) -> HandRank {
        std::assert_eq!(str.len(), 5);

        let first = str.as_bytes()[0] as char;
        let first_matches = str[1..]
            .match_indices(first)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        let initial_rank = match first_matches.len() {
            4 => Rank::Five,
            3 => Rank::Four,
            2 => {
                let remainder = str.replace(first, "");
                if remainder.as_bytes()[0] == remainder.as_bytes()[1] {
                    Rank::FullHouse
                } else {
                    Rank::Three
                }
            }
            1 => {
                let remainder = str.replace(first, "");
                let one_two = remainder.as_bytes()[0] == remainder.as_bytes()[1];
                let two_three = remainder.as_bytes()[1] == remainder.as_bytes()[2];
                let one_three = remainder.as_bytes()[0] == remainder.as_bytes()[2];
                if one_two && two_three {
                    Rank::FullHouse
                } else if one_two || two_three || one_three {
                    Rank::Two
                } else {
                    Rank::One
                }
            }
            0 => {
                let remainder = &str[1..];
                let second = remainder.as_bytes()[0] as char;
                let second_matches = remainder[1..]
                    .match_indices(second)
                    .map(|(idx, _)| idx)
                    .collect::<Vec<_>>();
                match second_matches.len() {
                    3 => Rank::Four,
                    2 => Rank::Three,
                    1 => {
                        let remainder = remainder.replace(second, "");
                        if remainder.as_bytes()[0] == remainder.as_bytes()[1] {
                            Rank::Two
                        } else {
                            Rank::One
                        }
                    }
                    0 => {
                        let remainder = &remainder[1..];
                        let one_two = remainder.as_bytes()[0] == remainder.as_bytes()[1];
                        let two_three = remainder.as_bytes()[1] == remainder.as_bytes()[2];
                        let one_three = remainder.as_bytes()[0] == remainder.as_bytes()[2];
                        if one_two && two_three {
                            Rank::Three
                        } else if one_two || two_three || one_three {
                            Rank::One
                        } else {
                            Rank::High
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        let joker_count = str.matches('J').count();
        let rank = match joker_count {
            4 => Rank::Five,
            3 => match initial_rank {
                Rank::FullHouse => Rank::Five,
                Rank::Three => Rank::Four,
                _ => unreachable!(),
            },
            2 => match initial_rank {
                Rank::FullHouse => Rank::Five,
                Rank::Two => Rank::Four,
                Rank::One => Rank::Three,
                _ => unreachable!(),
            },
            1 => match initial_rank {
                Rank::Four => Rank::Five,
                Rank::Three => Rank::Four,
                Rank::Two => Rank::FullHouse,
                Rank::One => Rank::Three,
                Rank::High => Rank::One,
                _ => unreachable!(),
            },
            _ => initial_rank,
        };

        let cards = str
            .as_bytes()
            .iter()
            .map(|c| Card::from_char(*c as char).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let hand = Hand { cards };
        HandRank { hand, rank }
    }
}

fn main() {
    let mut hands = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split_whitespace();
            let hand_rank = HandRank::from_str(line.next().unwrap());
            let bid: u64 = line.next().unwrap().parse().unwrap();
            (hand_rank, bid)
        })
        .collect::<Vec<_>>();
    hands.sort();

    //println!("{:#?}", hands);

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, bid))| acc + (bid * (idx + 1) as u64));

    println!("{winnings}")
}
