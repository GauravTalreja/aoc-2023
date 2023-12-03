fn main() {
    let empty = "";
    let numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let letters = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let sum = std::io::stdin()
        .lines()
        .fold(0, |acc, line| {
            if let Ok(line) = line {
                let first_number = line.find(numbers);
                let (first_letter, word) = letters.iter()
                    .map(|num| (line.find(num).unwrap_or(usize::MAX), num))
                    .min_by(|(i1, _), (i2, _)| i1.cmp(i2))
                    .unwrap_or((usize::MAX, &&empty));
                let c;
                if let Some(first_number) = first_number {
                    if first_number <= first_letter {
                        c = line.as_bytes()[first_number] as char;
                    } else {
                        c = numbers[letters.iter().position(|&num| num == *word).unwrap()];    
                    }
                } else {
                    c = numbers[letters.iter().position(|&num| num == *word).unwrap()];    
                }
                let first = c.to_digit(10).unwrap() * 10;
                
                
                let second_number = line.rfind(numbers);
                let (second_letter, word) = letters.iter()
                    .map(|num| (line.rfind(num).unwrap_or(usize::MIN), num))
                    .max_by(|(i1, _), (i2, _)| i1.cmp(i2))
                    .unwrap_or((usize::MIN, &&empty));
                let c;
                if let Some(second_number) = second_number {
                    if  second_number >= second_letter {
                        c = line.as_bytes()[second_number] as char;
                    } else {
                        c = numbers[letters.iter().position(|&num| num == *word).unwrap()];    
                    }
                } else {
                    c = numbers[letters.iter().position(|&num| num == *word).unwrap()];    
                }
                let second = c.to_digit(10).unwrap();
                
                return acc + first + second;
            }
            acc
        });
    println!("{sum}");
}

