#[derive(Debug)]
struct Car {
    brand: String,
    model: String,
    year: u16,
}

struct CarBuilder {
    brand: String,
    model: String,
    year: Option<u16>,
}

impl CarBuilder {
    fn new(brand: &str, model: &str) -> Self {
        CarBuilder {
            brand: brand.to_string(),
            model: model.to_string(),
            year: None,
        }
    }

    fn year(mut self, year: u16) -> Self {
        self.year = Some(year);
        self
    }

    fn build(self) -> Car {
        Car {
            brand: self.brand,
            model: self.model,
            year: self.year.unwrap_or(2024),
        }
    }
}

fn main() {
    let car = CarBuilder::new("Toyota", "Corolla")
        .year(2023)
        .build();

    println!("{:?}", car);
}
