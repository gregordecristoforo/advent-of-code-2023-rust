mod day1;
mod day2;
mod day3;
mod day4;
mod day6;

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

    println!("Day 3:");
    if let Err(e) = day3::gears::solutions() {
        eprintln!("Error: {}", e);
    }

    println!("Day 4:");
    if let Err(e) = day4::cards::solution_part_1() {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = day4::cards::solution_part_2() {
        eprintln!("Error: {}", e);
    }

    println!("Day 6:");
    if let Err(e) = day6::races::solutions() {
        eprintln!("Error: {}", e);
    }
}
