use std::time::Instant;

fn main() {
    let start = Instant::now();
    match solve_day6_puzzle(&1) {
        Ok(result) => println!("Total: {result}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    let end = Instant::now();
    let elapsed = end - start;
    println!("Execution time: {elapsed:?}");
}

fn solve_day6_puzzle(part: &u8) -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("day6_data.txt")?;
    let result = solve_puzzle(&input)?;
    Ok(result)
}

fn solve_puzzle(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let time_str = input.lines().next().ok_or("Could not read first line.")?;
    let times: Vec<u32> = time_str
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();
    let distance_str = input
        .lines()
        .skip(1)
        .next()
        .ok_or("Could not read second line.")?;
    let distances: Vec<u32> = distance_str
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let mut acc: u32 = 1;
    let it = times.iter().zip(distances.iter());
    for (time, dist) in it {
        let mut t_acc: u32 = 0;

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

#[cfg(test)]
mod test {
    use crate::solve_puzzle;

    #[test]
    fn test_simple() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        assert_eq!(solve_puzzle(input).unwrap(), 288)
    }
}
