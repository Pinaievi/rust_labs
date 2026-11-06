// Тип палива / двигуна
#[derive(Debug, Clone, Copy)]
pub enum FuelType {
    Petrol,
    Diesel,
}

impl FuelType {
    pub fn name(&self) -> &str {
        match self {
            FuelType::Petrol => "бензиновий",
            FuelType::Diesel => "дизельний",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AirControl {
    Conditioner,
    ClimateControl,
}

impl AirControl {
    pub fn name(&self) -> &str {
        match self {
            AirControl::Conditioner => "кондиціонер",
            AirControl::ClimateControl => "клімат-контроль",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Interior {
    Leather,
    Standard,
}

impl Interior {
    pub fn name(&self) -> &str {
        match self {
            Interior::Leather => "шкіра",
            Interior::Standard => "стандартна",
        }
    }
}


pub struct Car {
    pub name: String,
    pub engine_volume: u16,
    pub engine_power: u16,
    pub fuel_type: FuelType,
    pub abs: bool,
    pub esp: bool,
    pub airbag_count: u8,
    pub trip_computer: bool,
    pub air_control: AirControl,
    pub interior: Interior,
    pub price: u32,
}

fn yes_no(value: bool) -> &'static str {
    if value { "так" } else { "ні" }
}

impl Car {
    pub fn print_info(&self) {
        println!("Комплектація: {}", self.name);
        println!(
            "Двигун: {}, {} см³, {} к.с.",
            self.fuel_type.name(),
            self.engine_volume,
            self.engine_power
        );
        println!("ABS: {}", yes_no(self.abs));
        println!("ESP: {}", yes_no(self.esp));
        println!("Подушок безпеки: {}", self.airbag_count);
        println!("Бортовий комп'ютер: {}", yes_no(self.trip_computer));
        println!("Клімат: {}", self.air_control.name());
        println!("Салон: {}", self.interior.name());
        println!("Вартість: {} $", self.price);
        println!();
    }
}