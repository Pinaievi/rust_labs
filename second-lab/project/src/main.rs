mod components;

use components::{Car, Director, FuelType};
use std::io::{self, Write};


fn get_complect(name: &str) -> Option<Car> {

    let normalized = name.trim().to_lowercase().replace('_', " ");

    match normalized.as_str() {
        "standard petrol" => Some(Director::standard(FuelType::Petrol)),
        "standard diesel" => Some(Director::standard(FuelType::Diesel)),
        "max petrol" => Some(Director::max(FuelType::Petrol)),
        "max diesel" => Some(Director::max(FuelType::Diesel)),
        _ => None,
    }
}

fn main() {
    println!("Доступні комплектації: standard petrol, standard diesel, max petrol, max diesel");
    println!("Введіть назву комплектації (exit - вихід):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).unwrap();
        if bytes == 0 {
            break; // кінець вводу
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        match get_complect(input) {
            Some(car) => car.print_info(),
            None => println!("Комплектацію \"{}\" не знайдено\n", input),
        }
    }
}