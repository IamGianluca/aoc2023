use std::collections::{HashMap, HashSet};

use regex::Regex;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    match solve_day4_puzzle(&2) {
        Ok(result) => println!("Total: {result}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    let end = Instant::now();
    let elapsed = end - start;
    println!("Execution time: {elapsed:?}");
}

fn solve_day4_puzzle(part: &u8) -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("day4_data.txt")?;

    let result = if *part == 1 {
        solve_part1_puzzle(&input)?
    } else {
        solve_part2_puzzle(&input)?
    };
    Ok(result)
}

fn solve_part1_puzzle(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let n_matches = get_num_matches(line)?;
        if n_matches == 0 {
            continue;
        } else {
            let score = 2u32.pow(n_matches.saturating_sub(1));
            sum += score
        }
    }
    Ok(sum)
}

fn solve_part2_puzzle(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum: u32 = 0;
    let mut state = HashMap::<u32, u32>::new();
    for (card_num, line) in input.lines().enumerate() {
        let card_num = (card_num + 1) as u32;
        if let Some(value) = state.get(&card_num) {
            state.insert(card_num, value + 1);
        } else {
            state.insert(card_num, 1);
        }
        let n_matches = get_num_matches(line)?;
        let n_cards_of_this_num = *state.get(&card_num).ok_or("Could not find card")?;
        println!("{}: {}", card_num, n_cards_of_this_num);
        for u in 1..n_matches + 1 {
            let other_card = card_num + u;
            if let Some(value) = state.get(&other_card) {
                state.insert(other_card, value + n_cards_of_this_num);
            } else {
                state.insert(other_card, n_cards_of_this_num);
            }
        }
        sum += n_cards_of_this_num
    }
    Ok(sum)
}

fn get_num_matches(line: &str) -> Result<u32, Box<dyn std::error::Error>> {
    println!("card: {}", line);
    let win_str_re = Regex::new(r":(.*?)\|")?;
    let my_str_re = Regex::new(r"\|(.*?)$")?;
    let num_re = Regex::new(r"(\d+)")?;

    let mut win_nums = HashSet::new();
    let mut my_nums = HashSet::new();

    if let Some(winnnig_line) = win_str_re.captures(&line) {
        if let Some(winning_str) = winnnig_line.get(1) {
            let winning_str = winning_str.as_str();
            for num in num_re.find_iter(&winning_str) {
                let num = num.as_str().parse::<u32>()?;
                win_nums.insert(num);
            }
        }
        println!("winning numbers: {:?}", win_nums);
    };
    if let Some(my_line) = my_str_re.captures(&line) {
        if let Some(my_str) = my_line.get(1) {
            let my_str = my_str.as_str();
            for num in num_re.find_iter(&my_str) {
                let num = num.as_str().parse::<u32>()?;
                my_nums.insert(num);
            }
        }
        println!("my numbers: {:?}", my_nums);
    };
    let common_nums = win_nums.intersection(&my_nums);
    println!("numbers in common: {:?}", common_nums);
    let n_matches = common_nums.count() as u32;
    Ok(n_matches)
}

#[cfg(test)]
mod test {
    use crate::{solve_day4_puzzle, solve_part1_puzzle, solve_part2_puzzle};

    #[test]
    fn test_case1_part1() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(solve_part1_puzzle(&line).unwrap(), 8);
    }

    #[test]
    fn test_case2_part1() {
        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        assert_eq!(solve_part1_puzzle(&line).unwrap(), 2);
    }

    #[test]
    fn test_case3_part1() {
        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        assert_eq!(solve_part1_puzzle(&line).unwrap(), 2);
    }

    #[test]
    fn test_case4_part1() {
        let line = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        assert_eq!(solve_part1_puzzle(&line).unwrap(), 1);
    }

    #[test]
    fn test_case5_part1() {
        let line = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        assert_eq!(solve_part1_puzzle(&line).unwrap(), 0);
    }

    #[test]
    fn test_case6_part1() {
        let line = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve_part1_puzzle(&line).unwrap(), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_day4_puzzle(&1).unwrap(), 20_107)
    }

    #[test]
    fn test_part2_small() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve_part2_puzzle(&input).unwrap(), 30)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_day4_puzzle(&2).unwrap(), 8_172_507)
    }
}
