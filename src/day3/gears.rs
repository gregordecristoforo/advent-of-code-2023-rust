use std::fs;
use std::io;

fn find_numbers(line: &str) -> Vec<(usize, usize, usize)> {
    let mut numbers = Vec::new();
    let regex = regex::Regex::new(r"\d+").unwrap();

    for m in regex.find_iter(line) {
        numbers.push((m.as_str().parse().unwrap(), m.start(), m.end() - 1));
    }

    numbers
}

fn symbol_positions(lines: &Vec<&str>) -> Vec<(usize, usize)> {
    let mut symbols = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                symbols.push((row, col));
            }
        }
    }

    symbols
}

fn aura(row: usize, start: usize, end: usize) -> Vec<(usize, usize)> {
    let mut l = Vec::new();

    l.push((row, start.checked_sub(1).unwrap_or(usize::MAX)));
    l.push((row, end + 1));

    for i in start.checked_sub(1).unwrap_or(usize::MAX)..=end + 1 {
        l.push((row.checked_sub(1).unwrap_or(usize::MAX), i));
        l.push((row + 1, i));
    }

    l
}

fn aura_touches_symbol(
    row: usize,
    start: usize,
    end: usize,
    symbols: &Vec<(usize, usize)>,
) -> bool {
    aura(row, start, end)
        .iter()
        .any(|&position| symbols.contains(&position))
}

pub fn solutions() -> io::Result<()> {
    let content = fs::read_to_string("/home/gregor/git/adventofcode-2023-rust/src/day3/data.txt")
        .expect("Error reading the file");
    let lines: Vec<&str> = content.lines().collect();

    let symbols = symbol_positions(&lines);
    let mut gears: std::collections::HashMap<(usize, usize), Vec<usize>> =
        symbols.iter().cloned().map(|p| (p, Vec::new())).collect();

    let mut result_part1 = 0;

    for (row, line) in lines.iter().enumerate() {
        for (number, start, end) in find_numbers(line) {
            if aura_touches_symbol(row, start, end, &symbols) {
                result_part1 += number;
            }
            for position in aura(row, start, end) {
                if let Some(numbers) = gears.get_mut(&position) {
                    numbers.push(number);
                }
            }
        }
    }

    println!("Result part 1: {}", result_part1);

    let result_part2: usize = gears
        .values()
        .filter(|&numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .sum();
    println!("Result part 2: {}", result_part2);

    Ok(())
}
