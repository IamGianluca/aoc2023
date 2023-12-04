fn first_digit(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_digit(10))
}
fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();

    let mut acc = Vec::new();
    for line in input.lines() {
        let first = first_digit(&line).unwrap();
        let reversed: String = line.chars().rev().collect();
        let last = first_digit(&reversed).unwrap();

        let mut tmp = String::new();
        tmp.push(first);
        tmp.push(last);

        let total: u32 = tmp.parse().unwrap();
        acc.push(total);
        println!("{line}: {first} and {last} = {total}")
    }
    let sum: u32 = acc.iter().sum();
    println!("Total: {sum}")
}
