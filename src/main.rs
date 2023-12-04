mod day1;
mod day2;

fn main() {
    println!("Day 1:");
    if let Err(e) = day1::extractor::solution_part_1() {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = day1::extractor::solution_part_2() {
        eprintln!("Error: {}", e);
    }

    println!("Day 2:");
    if let Err(e) = day2::game::solution_part_1() {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = day2::game::solution_part_2() {
        eprintln!("Error: {}", e);
    }
}
