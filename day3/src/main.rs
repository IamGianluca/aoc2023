use std::cmp::{max, min};

use regex::Regex;

fn main() {
    match solve_puzzle() {
        Ok(sum) => println!("Total: {sum}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn solve_puzzle() -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data.txt")?;
    let mut schema = Vec::new();
    for line in input.lines() {
        schema.push(line);
    }
    let sum = solve_part1(&schema)?;
    Ok(sum)
}

fn solve_part1(schema: &Vec<&str>) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum: u32 = 0;

    let (max_rows, max_cols) = (schema.len() - 1, schema[0].len() - 1);
    let max_rows: i32 = max_rows as i32;
    let max_cols: i32 = max_cols as i32;

    for line in schema.iter() {
        println!("{:?}", line);
    }

    for (i, line) in schema.iter().enumerate() {
        println!("current line no. {}: {}", i, line);
        let re = Regex::new(r"\d+")?;
        for mtch in re.find_iter(&line) {
            let (start, end) = (mtch.start(), mtch.end() - 1);
            let start = start as i32;
            let end = end as i32;
            println!("num {}, start {}, end {}", mtch.as_str(), start, end);

            // extract borders
            let row_idx: i32 = i as i32;
            let x1 = max(0, row_idx - 1) as usize;
            let x2 = min(max_rows, row_idx + 1) as usize;
            let y1 = max(0, start - 1) as usize;
            let y2 = min(max_cols, end + 1) as usize;
            println!("({} x {}), ({} x {})", x1, y1, x2, y2);

            let mut quadrant: Vec<String> = Vec::new();
            for row_idx in x1..x2 + 1 {
                let subrow = &schema[row_idx];
                let string = &subrow;
                let slice = &string[y1..y2 + 1];
                println!("--> {:?}", slice);

                quadrant.push(slice.to_string());
            }

            let quadrant: String = quadrant.join("");
            if contains_symbol(&quadrant) {
                println!("we found a match");
                let num = mtch.as_str().parse::<u32>()?;
                sum += num
            }
        }
    }
    Ok(sum)
}

fn contains_symbol(s: &str) -> bool {
    let symbols = "#%&*+-/=@$";
    s.chars().any(|c| symbols.contains(*&c))
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_puzzle};

    #[test]
    fn test_extra_small_match() {
        let mut schema = Vec::new();
        schema.push("467.");
        schema.push("...*");
        schema.push("..35");
        assert_eq!(solve_part1(&schema).unwrap(), 467 + 35)
    }

    #[test]
    fn test_extra_small_no_match() {
        let mut schema = Vec::new();
        schema.push(".114..");
        schema.push("......");
        schema.push("..633.");
        assert_eq!(solve_part1(&schema).unwrap(), 0)
    }

    #[test]
    fn test_small() {
        let mut schema = Vec::new();
        schema.push("467..114..");
        schema.push("...*......");
        schema.push("..35..633.");
        schema.push("......#...");
        assert_eq!(solve_part1(&schema).unwrap(), 467 + 35 + 633)
    }

    #[test]
    fn test_large() {
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
        assert_eq!(solve_puzzle().unwrap(), 536_576);
    }
}
