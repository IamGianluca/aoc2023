use std::fmt;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    match solve_day1_puzzle(&2) {
        Ok(result) => println!("Total: {result}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    let end = Instant::now();
    let elapsed = end - start;
    println!("Execution time: {elapsed:?}");
}

fn solve_day1_puzzle(part: &i8) -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("day1_data.txt")?;
    let mut sum: u32 = 0;

    for line in input.lines() {
        let number = if *part == 1 {
            solve_part1_puzzle(line)?
        } else {
            solve_part2_puzzle(line)?
        };
        sum += number;
    }
    Ok(sum)
}

fn solve_part1_puzzle(line: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let first = first_digit(&line)?;
    let reversed: String = line.chars().rev().collect();
    let last = first_digit(&reversed)?;

    let combined = format!("{}{}", first, last);
    let number: u32 = combined.parse()?;
    Ok(number)
}

fn first_digit(s: &str) -> Result<char, OutOfRangeError> {
    s.chars().find(|c| c.is_digit(10)).ok_or(OutOfRangeError)
}

fn solve_part2_puzzle(line: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let first = extract_digits(&line, false)?;
    let reversed: String = line.chars().rev().collect();
    let last = extract_digits(&reversed, true)?;

    let combined = format!("{}{}", first, last);
    let number: u32 = combined.parse()?;
    Ok(number)
}

fn extract_digits(s: &str, reversed: bool) -> Result<char, OutOfRangeError> {
    for i in 0..s.len() {
        let substring: String = if reversed {
            let res = s.chars().take(i + 1).collect::<String>();
            res.chars().rev().collect::<String>()
        } else {
            s.chars().take(i + 1).collect::<String>()
        };

        if let Some(c) = s.chars().nth(i) {
            if c.is_digit(10) {
                return Ok(c);
            }
        }
        match parse_written_numbers(&substring) {
            Some(digit) => {
                return Ok(digit);
            }
            None => continue,
        }
    }
    Err(OutOfRangeError)
}

fn parse_written_numbers(s: &str) -> Option<char> {
    match s {
        _ if s.contains("one") => Some('1'),
        _ if s.contains("two") => Some('2'),
        _ if s.contains("three") => Some('3'),
        _ if s.contains("four") => Some('4'),
        _ if s.contains("five") => Some('5'),
        _ if s.contains("six") => Some('6'),
        _ if s.contains("seven") => Some('7'),
        _ if s.contains("eight") => Some('8'),
        _ if s.contains("nine") => Some('9'),
        &_ => None,
    }
}

#[derive(Debug, PartialEq)]
struct OutOfRangeError;

// Implementing the Display trait.
impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of range error")
    }
}

// Implementing the Error trait.
impl std::error::Error for OutOfRangeError {}

#[cfg(test)]
mod test {
    use crate::{solve_day1_puzzle, solve_part2_puzzle};

    #[test]
    fn test_one() {
        assert_eq!(solve_part2_puzzle("two1nine").unwrap(), 29);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve_part2_puzzle("eightwothree").unwrap(), 83);
    }

    #[test]
    fn test_three() {
        assert_eq!(solve_part2_puzzle("abcone2threexyz").unwrap(), 13);
    }

    #[test]
    fn test_four() {
        assert_eq!(solve_part2_puzzle("xtwone3four").unwrap(), 24);
    }

    #[test]
    fn test_five() {
        assert_eq!(solve_part2_puzzle("4nineeightseven2").unwrap(), 42);
    }

    #[test]
    fn test_six() {
        assert_eq!(solve_part2_puzzle("zoneight234").unwrap(), 14);
    }

    #[test]
    fn test_seven() {
        assert_eq!(solve_part2_puzzle("7pqrstsixteen").unwrap(), 76);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_day1_puzzle(&1).unwrap(), 54_304)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_day1_puzzle(&2).unwrap(), 54_418)
    }
}
