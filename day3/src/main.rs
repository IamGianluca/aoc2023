use std::{
    cmp::{max, min},
    collections::HashMap,
};

use regex::Regex;

fn main() {
    match solve_puzzle(2) {
        Ok(sum) => println!("Total: {sum}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn solve_puzzle(part: u8) -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data.txt")?;
    let mut schema = Vec::new();
    for line in input.lines() {
        schema.push(line);
    }
    let sum = if part == 1 {
        solve_part1(&schema)?
    } else {
        solve_part2(&schema)?
    };
    Ok(sum)
}

fn solve_part1(schema: &Vec<&str>) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum: u32 = 0;

    let re = Regex::new(r"\d+")?;

    let max_rows = schema.len() as i32 - 1;
    let max_cols = schema[0].len() as i32 - 1;

    for (i, line) in schema.iter().enumerate() {
        for mtch in re.find_iter(&line) {
            let start = mtch.start() as i32;
            let end = mtch.end() as i32 - 1;

            let row_idx: i32 = i as i32;
            let x1 = max(0, row_idx - 1) as usize;
            let x2 = min(max_rows, row_idx + 1) as usize;
            let y1 = max(0, start - 1) as usize;
            let y2 = min(max_cols, end + 1) as usize;
            let x_range = x1..=x2;
            let y_range = y1..=y2;

            let quadrant: Vec<String> = x_range
                .map(|x| schema[x as usize][y_range.clone()].to_string())
                .collect();

            let quadrant_flat: String = quadrant.join("");
            let symbols = "#%&*+-/=@$";
            if contains_symbol(&quadrant_flat, symbols) {
                println!("we found a match");
                sum += mtch.as_str().parse::<u32>()?;
            }
        }
    }
    Ok(sum)
}

fn contains_symbol(s: &str, symbols: &str) -> bool {
    s.chars().any(|c| symbols.contains(*&c))
}

fn solve_part2(schema: &Vec<&str>) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum: u32 = 0;
    let mut gears: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

    let re = Regex::new(r"\d+")?;
    let re2 = Regex::new(r"\*")?;

    let max_rows = schema.len() as i32 - 1;
    let max_cols = schema[0].len() as i32 - 1;

    for (i, line) in schema.iter().enumerate() {
        for mtch in re.find_iter(&line) {
            let start = mtch.start() as i32;
            let end = mtch.end() as i32 - 1;

            let row_idx: i32 = i as i32;
            let x1 = max(0, row_idx - 1) as usize;
            let x2 = min(max_rows, row_idx + 1) as usize;
            let y1 = max(0, start - 1) as usize;
            let y2 = min(max_cols, end + 1) as usize;
            let x_range = x1..=x2;
            let y_range = y1..=y2;

            let quadrant: Vec<String> = x_range
                .map(|x| schema[x as usize][y_range.clone()].to_string())
                .collect();

            let quadrant_flat: String = quadrant.join("");
            if quadrant_flat.contains("*") {
                let mut key: Option<(u32, u32)> = None;
                for (ii, quadrant_row) in quadrant.iter().enumerate() {
                    if contains_symbol(quadrant_row, "*") {
                        let x_coord = (x1 + ii) as u32;

                        let m = re2.find(quadrant_row).ok_or("Regex pattern not found");
                        let y_delta = m.unwrap().start() as usize;
                        let y_coord = (y1 + y_delta) as u32;
                        key = Some((x_coord, y_coord));
                        break;
                    }
                }

                let num = mtch.as_str().parse::<u32>()?;
                let value = num;
                println!("key: {:?}, value: {}", key, value);

                gears
                    .entry(key.unwrap())
                    .and_modify(|vec| vec.push(value)) // if key exists, modify the existing Vec
                    .or_insert_with(|| vec![value]); // if key does not exist, insert a new Vec
            }
        }
    }
    for value in gears.values() {
        if value.len() == 2 {
            sum += value.iter().product::<u32>();
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_part2, solve_puzzle};

    #[test]
    fn test_extra_small_match_part1() {
        let mut schema = Vec::new();
        schema.push("467.");
        schema.push("...*");
        schema.push("..35");
        assert_eq!(solve_part1(&schema).unwrap(), 467 + 35)
    }

    #[test]
    fn test_extra_small_no_match_part1() {
        let mut schema = Vec::new();
        schema.push(".114..");
        schema.push("......");
        schema.push("..633.");
        assert_eq!(solve_part1(&schema).unwrap(), 0)
    }

    #[test]
    fn test_small_part1() {
        let mut schema = Vec::new();
        schema.push("467..114..");
        schema.push("...*......");
        schema.push("..35..633.");
        schema.push("......#...");
        assert_eq!(solve_part1(&schema).unwrap(), 467 + 35 + 633)
    }

    #[test]
    fn test_large_part1() {
        let mut schema = Vec::new();
        schema.push("467..114..");
        schema.push("...*......");
        schema.push("..35..633.");
        schema.push("......#...");
        schema.push("617*......");
        schema.push(".....+.58.");
        schema.push("..592.....");
        schema.push("......755.");
        schema.push("...$.*....");
        schema.push(".664.598..");
        assert_eq!(
            solve_part1(&schema).unwrap(),
            467 + 35 + 633 + 617 + 592 + 755 + 664 + 598
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_puzzle(1).unwrap(), 536_576);
    }

    #[test]
    fn test_large_part2() {
        let mut schema = Vec::new();
        schema.push("467..114..");
        schema.push("...*......");
        schema.push("..35..633.");
        schema.push("......#...");
        schema.push("617*......");
        schema.push(".....+.58.");
        schema.push("..592.....");
        schema.push("......755.");
        schema.push("...$.*....");
        schema.push(".664.598..");
        assert_eq!(solve_part2(&schema).unwrap(), 467_835)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_puzzle(2).unwrap(), 75_741_499)
    }
}
