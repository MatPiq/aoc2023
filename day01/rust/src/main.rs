use std::collections::HashMap;

fn p1(input: &str) {
    println!(
        "{}",
        input.lines().fold(0, |acc, line| {
            let digits: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
            acc + (digits[0].to_string() + &digits[digits.len() - 1].to_string())
                .parse::<u32>()
                .unwrap()
        })
    );
}

fn p2(input: &str) {
    let digit_map = HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
        ("zero", "zero0zero"),
    ]);

    println!(
        "{}",
        input.lines().fold(0, |acc, line| {
            let mut line = line.to_string();
            digit_map.iter().for_each(|(k, v)| {
                line = line.replace(k, v);
            });

            let digits: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
            acc + (digits[0].to_string() + &digits[digits.len() - 1].to_string())
                .parse::<u32>()
                .unwrap()
        })
    );
}

fn main() {
    let input = include_str!("../../input");
    p1(input);
    p2(input);
}
