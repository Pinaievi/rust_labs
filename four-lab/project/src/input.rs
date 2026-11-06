use crate::equipment::{Brand, Direction, SkiMuneris};
use std::io::{self, Write};

pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    let bytes = io::stdin().read_line(&mut input).unwrap();
    if bytes == 0 {
        println!();
        std::process::exit(0);
    }
    input.trim().to_string()
}

pub fn read_number(prompt: &str, min: u32, max: u32) -> u32 {
    loop {
        match read_line(prompt).parse::<u32>() {
            Ok(value) if value >= min && value <= max => return value,
            _ => println!("Введіть число від {} до {}.", min, max),
        }
    }
}

pub fn read_snowboard(id: u32) -> (SkiMuneris, u32) {
    println!("Фірма: 1. Burton  2. Salomon  3. Nitro");
    let brand = match read_number("Ваш вибір: ", 1, 3) {
        1 => Brand::Burton,
        2 => Brand::Salomon,
        _ => Brand::Nitro,
    };

    let shoe_size = read_number("Розмір черевик (30-50): ", 30, 50) as u8;

    println!("Кріплення: 1. Праворуч  2. Ліворуч");
    let direction = match read_number("Ваш вибір: ", 1, 2) {
        1 => Direction::Right,
        _ => Direction::Left,
    };

    let quantity = read_number("Кількість сноубордів: ", 1, 100);

    (SkiMuneris::new(id, brand, shoe_size, direction), quantity)
}