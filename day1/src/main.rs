fn day1(part: i8) -> u32 {
    let input = std::fs::read_to_string("data.txt").unwrap();
    let mut sum: u32 = 0;

    for line in input.lines() {
        let number = if part == 1 {
            part1_solver(line)
        } else {
            part2_solver(line)
        };
        sum += number;
    }
    println!("Total: {sum}");
    sum
}

fn part1_solver(line: &str) -> u32 {
    let first = first_digit(&line).unwrap();
    let reversed: String = line.chars().rev().collect();
    let last = first_digit(&reversed).unwrap();

    let combined = format!("{}{}", first, last);
    let number: u32 = combined.parse().unwrap();
    number
}

fn first_digit(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_digit(10))
}

fn part2_solver(line: &str) -> u32 {
    println!("test case {}", line);
    let first = first_digit_incl_string(&line, false).unwrap();
    let reversed: String = line.chars().rev().collect();
    let last = first_digit_incl_string(&reversed, true).unwrap();

    let combined = format!("{}{}", first, last);
    let number: u32 = combined.parse().unwrap();
    number
}

fn first_digit_incl_string(s: &str, reversed: bool) -> Option<char> {
    for i in 0..s.len() {
        let substring: String = if reversed {
            s[0..i + 1].chars().rev().collect()
        } else {
            s[0..i + 1].to_string()
        };
        println!("processing {}", substring);

        if s.chars().nth(i).unwrap().is_digit(10) {
            println!("found digit {}", s.chars().nth(i).unwrap());
            return s.chars().nth(i);
        }
        match real_solver(&substring) {
            Some(digit) => {
                println!("found string {}", digit);
                return Some(digit);
            }
            None => continue,
        }
    }
    None
}

fn real_solver(s: &str) -> Option<char> {
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

fn main() {}

#[cfg(test)]
mod test {
    use crate::{day1, part2_solver};

    #[test]
    fn test_one() {
        assert_eq!(part2_solver("two1nine"), 29);
    }
    #[test]
    fn test_two() {
        assert_eq!(part2_solver("eightwothree"), 83);
    }
    #[test]
    fn test_three() {
        assert_eq!(part2_solver("abcone2threexyz"), 13);
    }
    #[test]
    fn test_four() {
        assert_eq!(part2_solver("xtwone3four"), 24);
    }
    #[test]
    fn test_five() {
        assert_eq!(part2_solver("4nineeightseven2"), 42);
    }
    #[test]
    fn test_six() {
        assert_eq!(part2_solver("zoneight234"), 14);
    }
    #[test]
    fn test_seven() {
        assert_eq!(part2_solver("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_part1() {
        assert_eq!(day1(1), 54_304)
    }

    #[test]
    fn test_part2() {
        assert_eq!(day1(2), 54_418)
    }
}
