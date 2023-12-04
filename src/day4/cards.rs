use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn get_points_per_card(card: &str) -> usize {
    let card = card.replace("\n", "");
    let card_parts: Vec<&str> = card.split(": ").collect();
    let (_, numbers) = (card_parts[0], card_parts[1]);

    let number_parts: Vec<&str> = numbers.split(" | ").collect();
    let (winning_numbers, your_numbers) = (number_parts[0], number_parts[1]);

    let winning_numbers: HashSet<&str> = winning_numbers.split_whitespace().collect();
    let your_numbers: HashSet<&str> = your_numbers.split_whitespace().collect();

    let matching_cards = winning_numbers.intersection(&your_numbers).count();
    matching_cards
}

pub fn solution_part_1() -> io::Result<()> {
    let file = File::open("/home/gregor/git/adventofcode-2023-rust/src/day4/data.txt")?;
    let reader = io::BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let card = line?;
        let matching_cards = get_points_per_card(&card);
        if matching_cards > 0 {
            result += 2u32.pow((matching_cards - 1) as u32);
        }
    }

    println!("Result part 1: {}", result);

    Ok(())
}

pub fn solution_part_2() -> io::Result<()> {
    let file = File::open("/home/gregor/git/adventofcode-2023-rust/src/day4/data.txt")?;
    let reader = io::BufReader::new(file);

    let mut number_of_cards = vec![1; 199];
    for (i, line) in reader.lines().enumerate() {
        let card = line?;
        let matching_cards = get_points_per_card(&card);
        for j in (i + 1)..(i + 1 + matching_cards) {
            number_of_cards[j] += number_of_cards[i];
        }
    }
    let result_part_2: usize = number_of_cards.iter().sum();

    println!("Result part 2: {}", result_part_2);

    Ok(())
}
