use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn solution_part_1() -> io::Result<()> {
    // Open the file
    let file = File::open("/home/gregor/git/adventofcode-2023-rust/src/day1/data.txt")?;
    let reader = io::BufReader::new(file);

    // Initialize result variable
    let mut result = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        // Unwrap the line or handle the error
        let row = line?;

        // Extract digits from the row
        let digits_only: String = row.chars().filter(|c| c.is_digit(10)).collect();

        // Parse the two-digit number
        if let Some(first_digit) = digits_only.chars().next() {
            if let Some(last_digit) = digits_only.chars().last() {
                if let Ok(two_digit) = format!("{}{}", first_digit, last_digit).parse::<i32>() {
                    // Update the result
                    result += two_digit;
                }
            }
        }
    }

    println!("Result part 1: {}", result);

    Ok(())
}

pub fn solution_part_2() -> io::Result<()> {
    let file = File::open("/home/gregor/git/adventofcode-2023-rust/src/day1/data.txt")?;
    let reader = io::BufReader::new(file);

    let mut word_to_number = HashMap::new();
    word_to_number.insert("one", "1");
    word_to_number.insert("two", "2");
    word_to_number.insert("three", "3");
    word_to_number.insert("four", "4");
    word_to_number.insert("five", "5");
    word_to_number.insert("six", "6");
    word_to_number.insert("seven", "7");
    word_to_number.insert("eight", "8");
    word_to_number.insert("nine", "9");

    let mut result = 0;
    for line in reader.lines() {
        let line = line?;
        result += calculate_calibration_value(&line, &word_to_number);
    }

    println!("Result part 2: {}", result);

    Ok(())
}
fn calculate_calibration_value(line: &str, word_to_number: &HashMap<&str, &str>) -> i32 {
    let mut numbers = Vec::new();

    for (index, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            numbers.push(c.to_string());
        } else {
            // Check if this is the beginning of a number string
            for (word, number) in word_to_number.iter() {
                if line[index..].starts_with(word) {
                    numbers.push(number.to_string());
                }
            }
        }
    }

    // Parse the first and last numbers and return their sum
    let result =
        numbers[0].parse::<i32>().unwrap() * 10 + numbers.last().unwrap().parse::<i32>().unwrap();
    result
}
