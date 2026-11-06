enum Communication {
    Water,
    Electricity,
    Gas,
    Heating,
    Sewage,
    Internet,
}

impl Communication {

    fn name(&self) -> &str {
        match self {
            Communication::Water => "водопостачання",
            Communication::Electricity => "електрика",
            Communication::Gas => "газ",
            Communication::Heating => "опалення",
            Communication::Sewage => "каналізація",
            Communication::Internet => "інтернет",
        }
    }
}

struct TechnicalSpecs {
    floors: u32,
    area: f64,
    communications: Vec<Communication>, 
}

impl TechnicalSpecs {
    fn new(floors: u32, area: f64, communications: Vec<Communication>) -> Self {
        TechnicalSpecs {
            floors,
            area,
            communications,
        }
    }

    fn floors(&self) -> u32 {
        self.floors
    }

    fn area(&self) -> f64 {
        self.area
    }

    fn add_communication(&mut self, communication: Communication) {
        self.communications.push(communication);
    }

    fn communications_list(&self) -> String {
        let mut names: Vec<&str> = Vec::new();
        for c in &self.communications {
            names.push(c.name());
        }
        names.join(", ")
    }
}

struct Address {
    city: String,
    street: String,
}

impl Address {
    fn new(city: &str, street: &str) -> Self {
        Address {
            city: city.to_string(),
            street: street.to_string(),
        }
    }

    fn full(&self) -> String {
        format!("{}, {}", self.city, self.street)
    }
}

struct Owner {
    company_name: String,
    address: Address, // Owner делегує роботу з адресою об'єкту Address
}

impl Owner {
    fn new(company_name: &str, address: Address) -> Self {
        Owner {
            company_name: company_name.to_string(),
            address,
        }
    }

    fn company_name(&self) -> &str {
        &self.company_name
    }

    fn address(&self) -> String {
        self.address.full()
    }
}

struct Building {
    specs: TechnicalSpecs, // делегат технічних характеристик
    owner: Owner,          // делегат власника
}

impl Building {
    fn new(specs: TechnicalSpecs, owner: Owner) -> Self {
        Building { specs, owner }
    }

    fn floors(&self) -> u32 {
        self.specs.floors()
    }

    fn area(&self) -> f64 {
        self.specs.area()
    }

    fn communications(&self) -> String {
        self.specs.communications_list()
    }

    fn add_communication(&mut self, communication: Communication) {
        self.specs.add_communication(communication);
    }

    fn owner_name(&self) -> &str {
        self.owner.company_name()
    }

    fn owner_address(&self) -> String {
        self.owner.address()
    }

    fn set_owner(&mut self, new_owner: Owner) {
        self.owner = new_owner;
    }

    fn print_info(&self) {
        println!("=== Інформація про будівлю ===");
        println!("Поверхів: {}", self.floors());
        println!("Площа: {:.1} м²", self.area());
        println!("Комунікації: {}", self.communications());
        println!("Власник: {}", self.owner_name());
        println!("Адреса власника: {}", self.owner_address());
        println!();
    }
}

fn main() {
    let specs = TechnicalSpecs::new(
        5,
        1250.5,
        vec![
            Communication::Water,
            Communication::Electricity,
            Communication::Sewage,
        ],
    );
    let owner = Owner::new("ТОВ \"Будінвест\"", Address::new("Львів", "вул. Зелена, 10"));

    let mut building = Building::new(specs, owner);

    building.print_info();

    building.add_communication(Communication::Gas);
    building.add_communication(Communication::Heating);
    building.add_communication(Communication::Internet);

    building.set_owner(Owner::new(
        "ПрАТ \"Київбуд\"",
        Address::new("Київ", "пр. Перемоги, 25"),
    ));

    println!("--- Після змін ---");
    building.print_info();
}