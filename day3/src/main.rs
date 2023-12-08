fn main() {
    println!("Hello, world!");
}

fn solve_puzzle(schema: &[[&str; 1]]) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(32)
}
#[cfg(test)]
mod test {
    use crate::solve_puzzle;

    #[test]
    fn test_simple_case() {
        let schema = [
            ["467..114.."],
            ["...*......"],
            ["..35..633."],
            ["......#..."],
            ["617*......"],
            [".....+.58."],
            ["..592....."],
            ["......755."],
            ["...$.*...."],
            [".664.598.."],
        ];
        assert_eq!(solve_puzzle(&schema).unwrap(), 4361)
    }
}
