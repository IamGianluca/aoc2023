use std::time::Instant;

fn main() {
    let start = Instant::now();
    match solve_day6_puzzle(&2) {
        Ok(result) => println!("Total: {result}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    let end = Instant::now();
    let elapsed = end - start;
    println!("Execution time: {elapsed:?}");
}

fn solve_day6_puzzle(part: &u8) -> Result<u64, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("day6_data.txt")?;
    let result = solve_puzzle(&input, part)?;
    Ok(result)
}

fn solve_puzzle(input: &str, part: &u8) -> Result<u64, Box<dyn std::error::Error>> {
    let u = input.lines().next().ok_or("error")?;
    let times = get_numbers(u, part)?;
    let u = input.lines().skip(1).next().ok_or("error")?;
    let dists = get_numbers(u, part)?;

    let mut acc: u64 = 1;
    let it = times.iter().zip(dists.iter());
    for (time, dist) in it {
        let mut t_acc: u64 = 0;

        for t in 1..*time {
            let t_left = time - t;
            let distance_travelled = t_left * t;

            if distance_travelled > *dist {
                t_acc += 1;
            }
        }
        acc *= t_acc;
    }
    Ok(acc)
}

fn get_numbers(input: &str, part: &u8) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let binding = get_after_colon(input).ok_or("Could not find anything after colon symbol.")?;
    let binding = &binding.as_str();

    let x = match part {
        1 => input,
        2 => binding,
        _ => "",
    };
    let result: Vec<u64> = x
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();
    Ok(result)
}

fn get_after_colon(input: &str) -> Option<String> {
    let split = input.split(":");
    let last = split.last()?;
    let all = last.replace(" ", "");
    Some(all)
}

#[cfg(test)]
mod test {
    use crate::{get_after_colon, solve_puzzle};

    #[test]
    fn test_simple_part1() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        assert_eq!(solve_puzzle(input, &1).unwrap(), 288)
    }

    #[test]
    fn test_split() {
        let input = "Time:      7  15   30";
        assert_eq!(get_after_colon(input).unwrap(), "71_530")
    }

    #[test]
    fn test_simple_part2() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        assert_eq!(solve_puzzle(input, &2).unwrap(), 71_503)
    }
}
