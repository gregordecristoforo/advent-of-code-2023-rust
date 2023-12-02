mod day1;
mod day2;

fn main() {
    let mut result = day1::extractor::solution_part_1();
    println!("Result is {:?}", result);

    result = day1::extractor::solution_part_2();
    println!("Result is {:?}", result);

    let mut result = day2::game::solution_part_1();
    println!("Result is {:?}", result);

    result = day2::game::solution_part_2();
    println!("Result is {:?}", result);
}
