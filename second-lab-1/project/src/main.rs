mod factories;
mod products;

use factories::{factory_for, PhoneFactory};
use products::Model;
use std::io::{self, Write};

fn ask(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn place_order(factory: &dyn PhoneFactory, model: Model) {
    let phone = factory.create_phone(model);
    let delivery = factory.create_delivery();

    println!();
    println!("=== Замовлення оформлено ===");
    phone.print_info();
    println!("Термін доставки: {} дн.", delivery.days());
    println!("Вартість доставки: {}$", delivery.cost());
    println!("Разом до сплати: {}$", phone.price() + delivery.cost());
    println!();
}

fn main() {
    println!("Інтернет-магазин телефонів");

    loop {
        println!("Країни виробники: 1 - США, 2 - Китай");
        let country = ask("Оберіть країну (або exit для виходу): ");

        if country.eq_ignore_ascii_case("exit") {
            break;
        }

        let factory = match factory_for(&country) {
            Some(f) => f,
            None => {
                println!("Невідома країна, спробуйте ще раз.\n");
                continue;
            }
        };

        println!("Моделі: 1 - Basic, 2 - Pro, 3 - Ultra");
        let model_text = ask("Оберіть модель: ");

        match Model::from_input(&model_text) {
            Some(model) => place_order(factory.as_ref(), model),
            None => println!("Невідома модель, спробуйте ще раз.\n"),
        }
    }
}