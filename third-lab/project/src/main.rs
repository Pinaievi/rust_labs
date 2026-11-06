use std::io::{self, Write};

#[derive(Clone)]
struct Owner {
    full_name: String,
    phone: String,
    apartment: Option<u32>,
}

impl Owner {
    fn new(full_name: &str, phone: &str, apartment: Option<u32>) -> Self {
        Owner {
            full_name: full_name.to_string(),
            phone: phone.to_string(),
            apartment,
        }
    }

    fn describe(&self) -> String {
        match self.apartment {
            Some(n) => format!("кв. {}: {}, тел. {}", n, self.full_name, self.phone),
            None => format!("{}, тел. {}", self.full_name, self.phone),
        }
    }
}


#[derive(Clone)]
struct BuildingInfo {
    area: f64,
    floors: u32,
    address: String,
}

trait Building {
    fn kind(&self) -> &str;
    fn info(&self) -> &BuildingInfo;
    fn info_mut(&mut self) -> &mut BuildingInfo;
    fn owners(&self) -> &Vec<Owner>;
    fn owners_mut(&mut self) -> &mut Vec<Owner>;
    fn multi_owner(&self) -> bool; 
    fn clone_box(&self) -> Box<dyn Building>;
    fn print_info(&self) {
        let info = self.info();
        println!("Тип: {}", self.kind());
        println!("Адреса: {}", info.address);
        println!("Площа: {:.1} м²", info.area);
        println!("Поверхів: {}", info.floors);
        println!("Власники:");
        if self.owners().is_empty() {
            println!("  (немає)");
        }
        for owner in self.owners() {
            println!("  - {}", owner.describe());
        }
        println!();
    }
}

#[derive(Clone)]
struct ApartmentBlock {
    info: BuildingInfo,
    owners: Vec<Owner>,
}

impl ApartmentBlock {
    fn prototype() -> Self {
        ApartmentBlock {
            info: BuildingInfo {
                area: 3200.0,
                floors: 9,
                address: String::from("типова адреса"),
            },
            owners: Vec::new(),
        }
    }
}

impl Building for ApartmentBlock {
    fn kind(&self) -> &str {
        "Багатоквартирний будинок"
    }
    fn info(&self) -> &BuildingInfo {
        &self.info
    }
    fn info_mut(&mut self) -> &mut BuildingInfo {
        &mut self.info
    }
    fn owners(&self) -> &Vec<Owner> {
        &self.owners
    }
    fn owners_mut(&mut self) -> &mut Vec<Owner> {
        &mut self.owners
    }
    fn multi_owner(&self) -> bool {
        true
    }
    fn clone_box(&self) -> Box<dyn Building> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct Cottage {
    info: BuildingInfo,
    owners: Vec<Owner>,
}

impl Cottage {
    fn prototype() -> Self {
        Cottage {
            info: BuildingInfo {
                area: 180.0,
                floors: 2,
                address: String::from("типова адреса"),
            },
            owners: Vec::new(),
        }
    }
}

impl Building for Cottage {
    fn kind(&self) -> &str {
        "Котедж"
    }
    fn info(&self) -> &BuildingInfo {
        &self.info
    }
    fn info_mut(&mut self) -> &mut BuildingInfo {
        &mut self.info
    }
    fn owners(&self) -> &Vec<Owner> {
        &self.owners
    }
    fn owners_mut(&mut self) -> &mut Vec<Owner> {
        &mut self.owners
    }
    fn multi_owner(&self) -> bool {
        false
    }
    fn clone_box(&self) -> Box<dyn Building> {
        Box::new(self.clone())
    }
}


struct PrototypeEntry {
    name: String,
    building: Box<dyn Building>,
}

struct PrototypeRegistry {
    entries: Vec<PrototypeEntry>,
}

impl PrototypeRegistry {
    fn new() -> Self {
        PrototypeRegistry {
            entries: vec![
                PrototypeEntry {
                    name: String::from("Типовий багатоквартирний будинок"),
                    building: Box::new(ApartmentBlock::prototype()),
                },
                PrototypeEntry {
                    name: String::from("Типовий котедж"),
                    building: Box::new(Cottage::prototype()),
                },
            ],
        }
    }

    fn count(&self) -> usize {
        self.entries.len()
    }

    fn name(&self, index: usize) -> &str {
        &self.entries[index].name
    }

    fn create(&self, index: usize) -> Option<Box<dyn Building>> {
        match self.entries.get(index) {
            Some(entry) => Some(entry.building.clone_box()),
            None => None,
        }
    }
}

fn read_line(prompt: &str) -> String {
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

fn read_f64(prompt: &str) -> f64 {
    loop {
        let text = read_line(prompt).replace(',', ".");
        match text.parse::<f64>() {
            Ok(value) if value > 0.0 => return value,
            _ => println!("Введіть додатне число."),
        }
    }
}

fn read_u32(prompt: &str) -> u32 {
    loop {
        match read_line(prompt).parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("Введіть ціле невід'ємне число."),
        }
    }
}

fn read_owner(with_apartment: bool) -> Owner {
    let full_name = read_line("ПІБ власника: ");
    let phone = read_line("Телефон: ");
    let apartment = if with_apartment {
        Some(read_u32("Номер квартири: "))
    } else {
        None
    };
    Owner::new(&full_name, &phone, apartment)
}

fn show_all(buildings: &Vec<Box<dyn Building>>) {
    if buildings.is_empty() {
        println!("Контейнер порожній.\n");
        return;
    }
    for (i, building) in buildings.iter().enumerate() {
        println!("--- Будинок №{} ---", i + 1);
        building.print_info();
    }
}

fn add_from_prototype(registry: &PrototypeRegistry, buildings: &mut Vec<Box<dyn Building>>) {
    println!("Оберіть прототип:");
    for i in 0..registry.count() {
        println!("{}. {}", i + 1, registry.name(i));
    }

    let choice = read_u32("Номер прототипу: ") as usize;
    if choice == 0 {
        println!("Невірний номер.\n");
        return;
    }

    let mut building = match registry.create(choice - 1) {
        Some(b) => b,
        None => {
            println!("Невірний номер.\n");
            return;
        }
    };

    building.info_mut().address = read_line("Адреса: ");
    let owner = read_owner(building.multi_owner());
    building.owners_mut().push(owner);

    buildings.push(building);
    println!("Будинок створено клонуванням прототипу й додано в контейнер.\n");
}

fn edit_owner(building: &mut dyn Building) {
    let count = building.owners().len();
    if count == 0 {
        println!("Власників немає.\n");
        return;
    }

    for (i, owner) in building.owners().iter().enumerate() {
        println!("{}. {}", i + 1, owner.describe());
    }

    let n = read_u32("Номер власника для заміни: ") as usize;
    if n == 0 || n > count {
        println!("Невірний номер.\n");
        return;
    }

    let new_owner = read_owner(building.multi_owner());
    building.owners_mut()[n - 1] = new_owner;
    println!("Дані власника змінено.\n");
}

fn edit_building(buildings: &mut Vec<Box<dyn Building>>) {
    if buildings.is_empty() {
        println!("Контейнер порожній.\n");
        return;
    }

    for (i, building) in buildings.iter().enumerate() {
        println!("{}. {} ({})", i + 1, building.info().address, building.kind());
    }

    let n = read_u32("Номер будинку: ") as usize;
    if n == 0 || n > buildings.len() {
        println!("Невірний номер.\n");
        return;
    }

    let building: &mut dyn Building = buildings[n - 1].as_mut();

    loop {
        println!("\nЩо змінити?");
        println!("1. Адресу");
        println!("2. Площу");
        println!("3. Кількість поверхів");
        println!("4. Замінити дані власника");
        println!("5. Додати власника квартири");
        println!("0. Назад");

        match read_line("Ваш вибір: ").as_str() {
            "1" => building.info_mut().address = read_line("Нова адреса: "),
            "2" => building.info_mut().area = read_f64("Нова площа (м²): "),
            "3" => building.info_mut().floors = read_u32("Нова кількість поверхів: "),
            "4" => edit_owner(building),
            "5" => {
                if building.multi_owner() {
                    let owner = read_owner(true);
                    building.owners_mut().push(owner);
                    println!("Власника додано.");
                } else {
                    println!("У котеджі лише один власник, скористайтесь заміною (пункт 4).");
                }
            }
            "0" => break,
            _ => println!("Невірний пункт меню."),
        }
    }
    println!();
}

fn main() {
    let registry = PrototypeRegistry::new();
    let mut buildings: Vec<Box<dyn Building>> = Vec::new();


    let mut block1 = registry.create(0).expect("прототип існує");
    block1.info_mut().address = String::from("м. Львів, вул. Зелена, 10");
    block1.owners_mut().push(Owner::new("Іваненко Петро Олегович", "+380501112233", Some(1)));
    block1.owners_mut().push(Owner::new("Коваль Марія Андріївна", "+380671234567", Some(2)));
    buildings.push(block1);

    let mut block2 = registry.create(0).expect("прототип існує");
    block2.info_mut().address = String::from("м. Київ, пр. Перемоги, 25");
    block2.info_mut().floors = 5;
    block2.info_mut().area = 1800.0;
    block2.owners_mut().push(Owner::new("Мельник Олена Ігорівна", "+380931112200", Some(7)));
    buildings.push(block2);

    let mut cottage = registry.create(1).expect("прототип існує");
    cottage.info_mut().address = String::from("с. Брюховичі, вул. Лісова, 3");
    cottage.owners_mut().push(Owner::new("Шевченко Андрій Васильович", "+380991234000", None));
    buildings.push(cottage);

    loop {
        println!("> Меню");
        println!("1. Переглянути всі будинки");
        println!("2. Додати будинок (клонування прототипу)");
        println!("3. Редагувати будинок");
        println!("0. Вихід");

        match read_line("Ваш вибір: ").as_str() {
            "1" => show_all(&buildings),
            "2" => add_from_prototype(&registry, &mut buildings),
            "3" => edit_building(&mut buildings),
            "0" => break,
            _ => println!("Невірний пункт меню.\n"),
        }
    }
}