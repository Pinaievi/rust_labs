use crate::equipment::{Brand, Direction, SkiMuneris};

pub trait RentalService {
    fn name(&self) -> &str;
    fn unit_price(&self, item: &SkiMuneris) -> f64;
    fn adjustment_price(&self, item: &SkiMuneris) -> f64;
}

fn base_price(brand: Brand) -> f64 {
    match brand {
        Brand::Burton => 600.0,
        Brand::Salomon => 500.0,
        Brand::Nitro => 450.0,
    }
}

pub struct CityRental;

impl RentalService for CityRental {
    fn name(&self) -> &str {
        "Міський прокат"
    }

    fn unit_price(&self, item: &SkiMuneris) -> f64 {
        base_price(item.brand)
    }

    fn adjustment_price(&self, item: &SkiMuneris) -> f64 {
        let mut price = 100.0;
        if item.shoe_size >= 44 {
            price += 20.0;
        }
        if let Direction::Left = item.direction {
            price += 15.0;
        }
        price
    }
}

pub struct ResortRental;

impl RentalService for ResortRental {
    fn name(&self) -> &str {
        "Прокат на курорті"
    }

    fn unit_price(&self, item: &SkiMuneris) -> f64 {
        base_price(item.brand) * 1.5
    }

    fn adjustment_price(&self, _item: &SkiMuneris) -> f64 {
        150.0
    }
}