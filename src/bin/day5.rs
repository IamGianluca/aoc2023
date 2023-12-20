fn main() {
    match solve_puzzle(&2) {
        Ok(result) => println!("Total: {result}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn solve_puzzle(part: &u8) -> Result<u64, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("day5_data.txt")?;
    let result = solve_part1(&input, part)?;
    Ok(result)
}

fn solve_part1(input: &str, part: &u8) -> Result<u64, Box<dyn std::error::Error>> {
    let first_line = input.lines().next().ok_or("Could not return first line")?;

    // extract seed numbers from first line
    println!("extracting seeds...");
    let (_, seed_str) = first_line.split_once(":").ok_or("Could not find char :")?;
    let seeds: Vec<u64> = match part {
        1 => Ok(extract_numbers(&seed_str)),
        2 => Ok(extract_numbers_part2(&seed_str)),
        _ => Err("only handlig values 1 or 2"),
    }?;

    // process file once
    let paragraphs: Vec<String> = input
        .lines()
        .skip(2)
        .collect::<Vec<_>>()
        .join("\n")
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();

    let mut maps = Vec::<Vec<[u64; 3]>>::new();
    for paragraph in paragraphs {
        let mut map = Vec::<[u64; 3]>::new();

        for line in paragraph.lines() {
            println!("{:?}", line);
            if line.contains("map") {
                continue;
            } else {
                let vec = extract_numbers(line);
                let arr: [u64; 3] = vec.try_into().ok().ok_or("error")?;
                map.push(arr);
            }
        }
        maps.push(map);
        println!("===========")
    }

    // solve location for each seed, one at a time
    let mut shortest = 999_999_999;
    let tot = seeds.len();

    println!("computing shortest path...");
    for (i, seed) in seeds.iter().enumerate() {
        let progress = (i as f64 / tot as f64) * 100.0;
        if i % 1_000 == 0 {
            // For example, print every 10 seeds
            println!("Progress: {:.2}%", progress);
        }

        let mut source = *seed;
        for map in &maps {
            for line in map {
                let [dest_start, source_start, range_length] = line;

                let min = source_start;
                let max = source_start + range_length;

                if min <= &source && source <= max {
                    let offset = source - min;
                    source = dest_start + offset;
                    break;
                }
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

fn extract_numbers_part2(text: &str) -> Vec<u64> {
    let nums: Vec<u64> = text
        .split_whitespace()
        .filter_map(|num_str| num_str.parse::<u64>().ok())
        .collect();

    let mut result = Vec::new();
    let mut last_num = 0;

    for (i, num) in nums.iter().enumerate() {
        if i % 2 != 0 {
            // range
            for offset in 0..*num {
                let offset = offset as u64;
                let something = last_num + offset;
                result.push(something);
            }
        } else {
            last_num = *num;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_puzzle};

    #[test]
    fn test_simple_part_1() {
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";
        assert_eq!(solve_part1(input, &1).unwrap(), 35)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_puzzle(&1).unwrap(), 157_211_394)
    }

    #[test]
    fn test_simple_part_2() {
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";
        assert_eq!(solve_part1(input, &2).unwrap(), 46)
    }

    // commented out due to long execution time
    // #[test]
    // fn test_part_2() {
    //     assert_eq!(solve_puzzle(&2).unwrap(), 50_855_035)
    // }
}
