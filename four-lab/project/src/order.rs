use crate::equipment::SkiMuneris;
use crate::rental::RentalService;

struct OrderLine {
    item: SkiMuneris,
    quantity: u32,
}

fn lines_total(service: &dyn RentalService, lines: &Vec<OrderLine>) -> f64 {
    let mut total = 0.0;
    for line in lines {
        let per_unit = service.unit_price(&line.item) + service.adjustment_price(&line.item);
        total += per_unit * line.quantity as f64;
    }
    total
}

fn total_quantity(lines: &Vec<OrderLine>) -> u32 {
    let mut total = 0;
    for line in lines {
        total += line.quantity;
    }
    total
}

fn print_lines(service: &dyn RentalService, lines: &Vec<OrderLine>) {
    if lines.is_empty() {
        println!("  (замовлення порожнє)");
    }
    for line in lines {
        let unit = service.unit_price(&line.item);
        let adjust = service.adjustment_price(&line.item);
        println!(
            "  #{} {}, розмір {}, кріплення {}: ({:.0} + {:.0}) x {} = {:.0} грн",
            line.item.id,
            line.item.brand.name(),
            line.item.shoe_size,
            line.item.direction.name(),
            unit,
            adjust,
            line.quantity,
            (unit + adjust) * line.quantity as f64
        );
    }
}

pub trait Order {
    fn add_snowboards(&mut self, item: SkiMuneris, quantity: u32);
    fn total_price(&self) -> f64;
    fn print_receipt(&self);
}

pub struct StandardOrder {
    service: Box<dyn RentalService>,
    lines: Vec<OrderLine>,
}

impl StandardOrder {
    pub fn new(service: Box<dyn RentalService>) -> Self {
        StandardOrder {
            service,
            lines: Vec::new(),
        }
    }
}

impl Order for StandardOrder {
    fn add_snowboards(&mut self, item: SkiMuneris, quantity: u32) {
        self.lines.push(OrderLine { item, quantity });
    }

    fn total_price(&self) -> f64 {
        lines_total(self.service.as_ref(), &self.lines)
    }

    fn print_receipt(&self) {
        println!("Замовлення (звичайне), {}", self.service.name());
        print_lines(self.service.as_ref(), &self.lines);
        println!("Разом: {:.0} грн\n", self.total_price());
    }
}

pub struct GroupOrder {
    service: Box<dyn RentalService>,
    lines: Vec<OrderLine>,
}

impl GroupOrder {
    pub fn new(service: Box<dyn RentalService>) -> Self {
        GroupOrder {
            service,
            lines: Vec::new(),
        }
    }

    fn has_discount(&self) -> bool {
        total_quantity(&self.lines) >= 5
    }
}

impl Order for GroupOrder {
    fn add_snowboards(&mut self, item: SkiMuneris, quantity: u32) {
        self.lines.push(OrderLine { item, quantity });
    }

    fn total_price(&self) -> f64 {
        let total = lines_total(self.service.as_ref(), &self.lines);
        if self.has_discount() {
            total * 0.9
        } else {
            total
        }
    }

    fn print_receipt(&self) {
        println!("Замовлення (групове), {}", self.service.name());
        print_lines(self.service.as_ref(), &self.lines);
        if self.has_discount() {
            println!("Групова знижка 10% застосована");
        }
        println!("Разом: {:.0} грн\n", self.total_price());
    }
}