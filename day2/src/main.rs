use std::collections::HashMap;

use regex::Regex;

fn main() {
    match solve_puzzle(2) {
        Ok(sum) => println!("Total: {sum}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn solve_puzzle(part: u8) -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data.txt")?;
    let mut sum = 0;

    for line in input.lines() {
        let number = if part == 1 {
            solve_part1(line)?
        } else {
            solve_part2(line)?
        };
        sum += number
    }
    Ok(sum)
}

fn solve_part1(scorecard: &str) -> Result<u32, Box<dyn std::error::Error>> {
    // return game number if the game is impossible, else return 0
    let game_number = extract_game_number(scorecard)?;
    match is_valid_scorecard(scorecard) {
        Ok(true) => Ok(game_number),
        Ok(false) => Ok(0),
        Err(_) => Ok(0),
    }
}

fn extract_game_number(s: &str) -> Result<u32, Box<dyn std::error::Error>> {
    // extract game number from scorecard string
    let re = Regex::new(r"^Game (?<game_number>\d+):")?;
    let caps = re.captures(s).ok_or("No match found")?;
    let game_digit = caps["game_number"].parse::<u32>()?;
    Ok(game_digit)
}

fn is_valid_scorecard(scorecard: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // return true if valid scorecard, e.g., at no point, more cubes of available ones were
    // extracted. else, return false
    println!("{}", scorecard);
    let color2max = HashMap::<&str, u32>::from([("red", 12), ("green", 13), ("blue", 14)]);

    for color in ["red", "blue", "green"] {
        let max_occurrences = color2max.get(color).unwrap();

        let regex_pattern = format!(r" (\d+) {}", color);
        let re = Regex::new(&regex_pattern)?;

        for (_, [count]) in re.captures_iter(scorecard).map(|c| c.extract()) {
            let count_digit = count.parse::<u32>()?;
            println!(
                "color: {}, count_digit: {}, max_occurrences {}",
                color, count_digit, *max_occurrences
            );
            if count_digit > *max_occurrences {
                println!("--> invalid scorecard <--");
                return Ok(false);
            }
        }
    }
    Ok(true)
}

// fn solve_part2(scorecard: &str) -> Result<u32, Box<dyn std::error::Error>> {
//     // return the product of the minimum number of cuber needed for each color to have a valid
//     // scorecard
//     let mut sum: u32 = 0;
//     for line in scorecard.lines() {
//         let number = solve_part2(line)?;
//         sum += number;
//     }
//     Ok(sum)
// }

fn solve_part2(scorecard: &str) -> Result<u32, Box<dyn std::error::Error>> {
    //     // return the product of the minimum number of cuber needed for each color to have a valid
    //     // scorecard
    let mut power = 1;
    for color in ["red", "blue", "green"] {
        let regex_pattern = format!(r" (\d+) {}", color);
        let re = Regex::new(&regex_pattern)?;

        let mut max: u32 = 0;
        for (_, [count]) in re.captures_iter(scorecard).map(|c| c.extract()) {
            let count = count.parse::<u32>()?;
            if count > max {
                max = count;
            }
        }
        power *= max;
    }
    Ok(power)
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_part2, solve_puzzle};

    #[test]
    fn test_one_part1() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(solve_part1(game).unwrap(), 1)
    }

    #[test]
    fn test_two_part1() {
        let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(solve_part1(game).unwrap(), 2)
    }

    #[test]
    fn test_three_part1() {
        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(solve_part1(game).unwrap(), 0)
    }

    #[test]
    fn test_four_part1() {
        let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(solve_part1(game).unwrap(), 0)
    }

    #[test]
    fn test_five_part1() {
        let game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve_part1(game).unwrap(), 5)
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_puzzle(1).unwrap(), 2006)
    }

    #[test]
    fn test_one_part2() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(solve_part2(game).unwrap(), 48)
    }

    #[test]
    fn test_two_part2() {
        let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(solve_part2(game).unwrap(), 12)
    }

    #[test]
    fn test_three_part2() {
        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(solve_part2(game).unwrap(), 1560)
    }

    #[test]
    fn test_four_part2() {
        let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(solve_part2(game).unwrap(), 630)
    }

    #[test]
    fn test_five_part2() {
        let game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve_part2(game).unwrap(), 36)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_puzzle(2).unwrap(), 84911)
    }
}
