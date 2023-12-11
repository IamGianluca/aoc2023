use std::collections::HashSet;

use regex::Regex;

fn main() {
    match solve_puzzle(1) {
        Ok(sum) => println!("Total: {sum}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn solve_puzzle(part: u8) -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data.txt")?;
    let mut sum: u32 = 0;

    for line in input.lines() {
        sum += if part == 1 {
            solve_part1(&line)?
        } else {
            solve_part2(&line)?
        };
    }
    Ok(sum)
}

fn solve_part1(line: &str) -> Result<u32, Box<dyn std::error::Error>> {
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
    if n_matches == 0 {
        Ok(0)
    } else {
        let score = 2u32.pow(n_matches.saturating_sub(1));
        Ok(score)
    }
}

fn solve_part2(line: &str) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(32)
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_puzzle};

    #[test]
    fn test_case1_part1() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(solve_part1(&line).unwrap(), 8);
    }

    #[test]
    fn test_case2_part1() {
        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        assert_eq!(solve_part1(&line).unwrap(), 2);
    }

    #[test]
    fn test_case3_part1() {
        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        assert_eq!(solve_part1(&line).unwrap(), 2);
    }

    #[test]
    fn test_case4_part1() {
        let line = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        assert_eq!(solve_part1(&line).unwrap(), 1);
    }

    #[test]
    fn test_case5_part1() {
        let line = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        assert_eq!(solve_part1(&line).unwrap(), 0);
    }

    #[test]
    fn test_case6_part1() {
        let line = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve_part1(&line).unwrap(), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_puzzle(1).unwrap(), 20_107)
    }
}
