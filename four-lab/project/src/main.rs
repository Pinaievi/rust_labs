mod equipment;
mod input;
mod order;
mod rental;

use input::{read_number, read_snowboard};
use order::{GroupOrder, Order, StandardOrder};
use rental::{CityRental, RentalService, ResortRental};

fn main() {
    println!("ПРОКАТ СНОУБОРДІВ");

    println!("Прокатна точка: 1. Міський прокат  2. Прокат на курорті");
    let service: Box<dyn RentalService> = match read_number("Ваш вибір: ", 1, 2) {
        1 => Box::new(CityRental),
        _ => Box::new(ResortRental),
    };

    println!("Тип замовлення: 1. Звичайне  2. Групове (знижка 10% від 5 шт.)");
    let mut order: Box<dyn Order> = match read_number("Ваш вибір: ", 1, 2) {
        1 => Box::new(StandardOrder::new(service)),
        _ => Box::new(GroupOrder::new(service)),
    };

    let mut next_id: u32 = 1;

    loop {
        println!("\n1. Додати сноуборди");
        println!("2. Показати замовлення і суму");
        println!("0. Завершити");

        match read_number("Ваш вибір: ", 0, 2) {
            1 => {
                let (item, quantity) = read_snowboard(next_id);
                next_id += 1;
                order.add_snowboards(item, quantity);
                println!("Додано. Поточна сума: {:.0} грн", order.total_price());
            }
            2 => order.print_receipt(),
            _ => break,
        }
    }

    println!("\nПідсумок:");
    order.print_receipt();
}