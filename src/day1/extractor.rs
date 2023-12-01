use std::fs::File;
use std::io::{self, BufRead};

pub fn solution_day_1() -> io::Result<()> {
    // Open the file
    let file = File::open("day1/data.txt")?;
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

    // Print the result
    println!("Result is {}", result);

    Ok(())
}
