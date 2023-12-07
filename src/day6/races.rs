use std::io;
use std::fs;

fn extract_numbers_part_1(line: &str) -> Vec<i64> {
    let mut values = vec![];

    for value in line.split(": ").flat_map(|split| split.split_whitespace()) {
        if value != ""{
            values.push(value);
        }
    }
    let mut numbers: Vec<i64> = vec![];

    for str in values[1..].to_vec(){
        let num: i64 = str.parse::<i64>().unwrap();
        numbers.push(num);
    }
    numbers
}

fn extract_numbers_part_2(line: &str) -> Vec<i64> {
    let values: String = line.split(": ").nth(1).unwrap().chars().filter(|&c| c != ' ').collect();
    let num: i64 = values.parse().unwrap();
    vec![num]
}

fn calculate_races(times: Vec<i64>, distances: Vec<i64>) -> u64 {
    let mut result = 1;

    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let mut beating_record = 0;
        for i in 1..time {
            if i * (time - i) > distance {
                beating_record += 1;
            }
        }

        result *= beating_record;
    }

    result
}

pub fn solutions() -> io::Result<()> {
    let content = fs::read_to_string("/home/gregor/git/adventofcode-2023-rust/src/day6/data.txt")?;
    let lines: Vec<&str> = content.lines().collect();

    let times = extract_numbers_part_1(lines[0]);
    let distances = extract_numbers_part_1(lines[1]);
    let result = calculate_races(times, distances);
    println!("Resutl part 1: {}", result);
    
    let times = extract_numbers_part_2(lines[0]);
    let distances = extract_numbers_part_2(lines[1]);
    let result = calculate_races(times, distances);
    println!("Resutl part 2: {}", result);
    Ok(())
}
