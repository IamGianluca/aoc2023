fn main() {
    match solve_puzzle() {
        Ok(result) => println!("Total: {result}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn solve_puzzle() -> Result<u64, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data.txt")?;
    let result = solve_part1(&input)?;
    Ok(result)
}

fn solve_part1(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let first_line = input.lines().next().ok_or("could not return first line")?;

    // extract seed numbers from first line
    let (_, seed_str) = first_line.split_once(":").ok_or("could not find char :")?;
    let seeds: Vec<u64> = extract_numbers(&seed_str);

    // create an array with the names of the different mappings we will encounter
    let xxx = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    // solve location for each seed, one at a time
    let mut shortest = 999_999_999;
    let mut ignore_flag = 0;

    for seed in seeds {
        let mut source = seed;
        for line in input.lines().skip(2) {
            // note: it is important that this if statement stays at the beginning of this
            // procedure
            if xxx.iter().any(|&s| s == line) {
                ignore_flag = 0;
                continue;
            }

            if line.is_empty() || ignore_flag == 1 {
                continue;
            }

            let vec = extract_numbers(line);
            let arr: [u64; 3] = vec.try_into().ok().ok_or("error")?;
            let [dest_start, source_start, range_length] = arr;

            let min = source_start;
            let max = source_start + range_length;

            if min <= source && source <= max {
                let offset = source - min;
                source = dest_start + offset;
                ignore_flag = 1;
            }
        }
        if source < shortest {
            shortest = source;
        }
    }

    Ok(shortest)
}

fn extract_numbers(text: &str) -> Vec<u64> {
    text.split_whitespace()
        .filter_map(|num_str| num_str.parse::<u64>().ok())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_puzzle};

    #[test]
    fn test_simple_part_1() {
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";
        assert_eq!(solve_part1(input).unwrap(), 35)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_puzzle().unwrap(), 157_211_394)
    }
}
