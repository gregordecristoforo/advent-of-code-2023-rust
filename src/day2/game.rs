use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Round {
    red: i32,
    blue: i32,
    green: i32,
}

impl Default for Round {
    fn default() -> Self {
        Round {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

fn parse_round_string(round_str: &str) -> Round {
    let mut round_obj = Round::default();

    for part in round_str.split(',') {
        let mut iter = part.trim().split_whitespace();
        if let (Some(quantity_str), Some(color)) = (iter.next(), iter.next()) {
            let quantity = quantity_str.parse().unwrap_or(0);
            match color.to_lowercase().as_str() {
                "red" => round_obj.red = quantity,
                "blue" => round_obj.blue = quantity,
                "green" => round_obj.green = quantity,
                _ => {}
            }
        }
    }

    round_obj
}

pub fn solution_part_1() -> io::Result<()> {
    let max_cubes: HashMap<&str, i32> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    let mut result_part1 = 0;
    if let Ok(file) = File::open("data.txt") {
        for (i, line) in io::BufReader::new(file).lines().enumerate() {
            let cleaned_string = line?.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
            let games: Vec<Round> = cleaned_string
                .split(';')
                .map(|s| parse_round_string(s))
                .collect();

            let invalid_rounds = games.iter().any(|round| {
                round.green > *max_cubes.get("green").unwrap_or(&0)
                    || round.red > *max_cubes.get("red").unwrap_or(&0)
                    || round.blue > *max_cubes.get("blue").unwrap_or(&0)
            });

            if !invalid_rounds {
                result_part1 += i + 1;
            }
        }
    }

    println!("Result part 1: {}", result_part1);
    Ok(())
}

pub fn solution_part_2() -> io::Result<()> {
    let mut result_part2 = 0;

    if let Ok(file) = File::open("data.txt") {
        for line in io::BufReader::new(file).lines() {
            let cleaned_string = line?.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
            let games: Vec<Round> = cleaned_string
                .split(';')
                .map(|s| parse_round_string(s))
                .collect();

            let mut max_values = Round::default();

            for round in &games {
                max_values.green = max_values.green.max(round.green);
                max_values.blue = max_values.blue.max(round.blue);
                max_values.red = max_values.red.max(round.red);
            }

            result_part2 += max_values.green * max_values.blue * max_values.red;
        }
    }

    println!("Result part 2: {}", result_part2);

    Ok(())
}
