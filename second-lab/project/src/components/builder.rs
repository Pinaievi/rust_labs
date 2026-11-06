use super::car::{AirControl, Car, FuelType, Interior};

pub struct CarBuilder {
    name: String,
    engine_volume: u16,
    engine_power: u16,
    fuel_type: FuelType,
    abs: bool,
    esp: bool,
    airbag_count: u8,
    trip_computer: bool,
    air_control: AirControl,
    interior: Interior,
    price: u32,
}

impl CarBuilder {
    // Початкові значення: "порожня" базова машина
    pub fn new() -> Self {
        CarBuilder {
            name: String::from("Без назви"),
            engine_volume: 1000,
            engine_power: 100,
            fuel_type: FuelType::Petrol,
            abs: false,
            esp: false,
            airbag_count: 0,
            trip_computer: false,
            air_control: AirControl::Conditioner,
            interior: Interior::Standard,
            price: 0,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn engine_volume(mut self, volume: u16) -> Self {
        self.engine_volume = volume;
        self
    }

    pub fn engine_power(mut self, power: u16) -> Self {
        self.engine_power = power;
        self
    }

    pub fn fuel_type(mut self, fuel_type: FuelType) -> Self {
        self.fuel_type = fuel_type;
        self
    }

    pub fn abs(mut self, on: bool) -> Self {
        self.abs = on;
        self
    }

    pub fn esp(mut self, on: bool) -> Self {
        self.esp = on;
        self
    }

    pub fn airbag_count(mut self, count: u8) -> Self {
        self.airbag_count = count;
        self
    }

    pub fn trip_computer(mut self, on: bool) -> Self {
        self.trip_computer = on;
        self
    }

    pub fn air_control(mut self, air_control: AirControl) -> Self {
        self.air_control = air_control;
        self
    }

    pub fn interior(mut self, interior: Interior) -> Self {
        self.interior = interior;
        self
    }

    pub fn price(mut self, price: u32) -> Self {
        self.price = price;
        self
    }

    pub fn build(self) -> Car {
        Car {
            name: self.name,
            engine_volume: self.engine_volume,
            engine_power: self.engine_power,
            fuel_type: self.fuel_type,
            abs: self.abs,
            esp: self.esp,
            airbag_count: self.airbag_count,
            trip_computer: self.trip_computer,
            air_control: self.air_control,
            interior: self.interior,
            price: self.price,
        }
    }
}

pub struct Director;

impl Director {
    pub fn standard(fuel: FuelType) -> Car {
        // Фіксована ціна для кожної з двох версій
        let price = match fuel {
            FuelType::Petrol => 18_000,
            FuelType::Diesel => 19_500,
        };

        CarBuilder::new()
            .name("Standard")
            .fuel_type(fuel)
            .engine_volume(1600)
            .engine_power(150)
            .abs(false)
            .esp(false)
            .airbag_count(1)
            .trip_computer(false)
            .air_control(AirControl::Conditioner)
            .interior(Interior::Standard)
            .price(price)
            .build()
    }

    pub fn max(fuel: FuelType) -> Car {
        let price = match fuel {
            FuelType::Petrol => 26_000,
            FuelType::Diesel => 27_500,
        };

        CarBuilder::new()
            .name("Max")
            .fuel_type(fuel)
            .engine_volume(2000)
            .engine_power(250)
            .abs(true)
            .esp(true)
            .airbag_count(2)
            .trip_computer(true)
            .air_control(AirControl::ClimateControl)
            .interior(Interior::Leather)
            .price(price)
            .build()
    }
}