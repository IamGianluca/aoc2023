fn first_digit(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_digit(10))
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();
    let mut sum: u32 = 0;

    for line in input.lines() {
        let first = first_digit(&line).unwrap();
        let reversed: String = line.chars().rev().collect();
        let last = first_digit(&reversed).unwrap();

        let combined = format!("{}{}", first, last);
        let number: u32 = combined.parse().unwrap();

        sum += number;
        println!("{line}: {first} and {last} = {number}")
    }
    println!("Total: {sum}")
}
