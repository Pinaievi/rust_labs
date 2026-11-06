use crate::products::{
    ChinaDelivery, ChinaPhone, Delivery, Model, Phone, UsaDelivery, UsaPhone,
};

pub trait PhoneFactory {
    fn create_phone(&self, model: Model) -> Box<dyn Phone>;
    fn create_delivery(&self) -> Box<dyn Delivery>;
}

pub struct UsaFactory;

impl PhoneFactory for UsaFactory {
    fn create_phone(&self, model: Model) -> Box<dyn Phone> {
        Box::new(UsaPhone::new(model))
    }

    fn create_delivery(&self) -> Box<dyn Delivery> {
        Box::new(UsaDelivery)
    }
}

pub struct ChinaFactory;

impl PhoneFactory for ChinaFactory {
    fn create_phone(&self, model: Model) -> Box<dyn Phone> {
        Box::new(ChinaPhone::new(model))
    }

    fn create_delivery(&self) -> Box<dyn Delivery> {
        Box::new(ChinaDelivery)
    }
}

pub fn factory_for(text: &str) -> Option<Box<dyn PhoneFactory>> {
    match text.trim().to_lowercase().as_str() {
        "1" | "сша" | "usa" => Some(Box::new(UsaFactory)),
        "2" | "китай" | "china" => Some(Box::new(ChinaFactory)),
        _ => None,
    }
}