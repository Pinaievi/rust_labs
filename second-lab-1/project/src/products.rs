#[derive(Debug, Clone, Copy)]
pub enum Model {
    Basic,
    Pro,
    Ultra,
}

impl Model {
    pub fn name(&self) -> &str {
        match self {
            Model::Basic => "Basic",
            Model::Pro => "Pro",
            Model::Ultra => "Ultra",
        }
    }


    pub fn base_price(&self) -> u32 {
        match self {
            Model::Basic => 300,
            Model::Pro => 600,
            Model::Ultra => 900,
        }
    }

    pub fn from_input(text: &str) -> Option<Model> {
        match text.trim().to_lowercase().as_str() {
            "1" | "basic" => Some(Model::Basic),
            "2" | "pro" => Some(Model::Pro),
            "3" | "ultra" => Some(Model::Ultra),
            _ => None,
        }
    }
}

pub trait Phone {
    fn model(&self) -> Model;
    fn country(&self) -> &str;
    fn price(&self) -> u32;

    fn print_info(&self) {
        println!("Телефон: {}", self.model().name());
        println!("Країна виробник: {}", self.country());
        println!("Вартість телефона: {}$", self.price());
    }
}

pub trait Delivery {
    fn days(&self) -> u32;
    fn cost(&self) -> u32;
}

pub struct UsaPhone {
    model: Model,
}

impl UsaPhone {
    pub fn new(model: Model) -> Self {
        UsaPhone { model }
    }
}

impl Phone for UsaPhone {
    fn model(&self) -> Model {
        self.model
    }

    fn country(&self) -> &str {
        "США"
    }

    fn price(&self) -> u32 {
        self.model.base_price() + 120
    }
}

pub struct UsaDelivery;

impl Delivery for UsaDelivery {
    fn days(&self) -> u32 {
        3
    }

    fn cost(&self) -> u32 {
        10
    }
}

pub struct ChinaPhone {
    model: Model,
}

impl ChinaPhone {
    pub fn new(model: Model) -> Self {
        ChinaPhone { model }
    }
}

impl Phone for ChinaPhone {
    fn model(&self) -> Model {
        self.model
    }

    fn country(&self) -> &str {
        "Китай"
    }

    fn price(&self) -> u32 {
        self.model.base_price() + 40
    }
}

pub struct ChinaDelivery;

impl Delivery for ChinaDelivery {
    fn days(&self) -> u32 {
        14
    }

    fn cost(&self) -> u32 {
        25
    }
}