trait Chair {
    fn sit_on(&self);
}

trait Sofa {
    fn lie_on(&self);
}

trait CoffeeTable {
    fn place_items(&self);
}

struct ModernChair;
struct VictorianChair;

impl Chair for ModernChair {
    fn sit_on(&self) {
        println!("Sitting on a modern chair.");
    }
}

impl Chair for VictorianChair {
    fn sit_on(&self) {
        println!("Sitting on a Victorian chair.");
    }
}

struct ModernSofa;
struct VictorianSofa;

impl Sofa for ModernSofa {
    fn lie_on(&self) {
        println!("Lying on a modern sofa.");
    }
}

impl Sofa for VictorianSofa {
    fn lie_on(&self) {
        println!("Lying on a Victorian sofa.");
    }
}

struct ModernCoffeeTable;
struct VictorianCoffeeTable;

impl CoffeeTable for ModernCoffeeTable {
    fn place_items(&self) {
        println!("Placing items on a modern coffee table.");
    }
}

impl CoffeeTable for VictorianCoffeeTable {
    fn place_items(&self) {
        println!("Placing items on a Victorian coffee table.");
    }
}

trait FurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair>;
    fn create_sofa(&self) -> Box<dyn Sofa>;
    fn create_coffee_table(&self) -> Box<dyn CoffeeTable>;
}

struct ModernFurnitureFactory;
struct VictorianFurnitureFactory;

impl FurnitureFactory for ModernFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(ModernChair)
    }
    fn create_sofa(&self) -> Box<dyn Sofa> {
        Box::new(ModernSofa)
    }
    fn create_coffee_table(&self) -> Box<dyn CoffeeTable> {
        Box::new(ModernCoffeeTable)
    }
}

impl FurnitureFactory for VictorianFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(VictorianChair)
    }
    fn create_sofa(&self) -> Box<dyn Sofa> {
        Box::new(VictorianSofa)
    }
    fn create_coffee_table(&self) -> Box<dyn CoffeeTable> {
        Box::new(VictorianCoffeeTable)
    }
}

fn main() {
    let modern_factory: Box<dyn FurnitureFactory> = Box::new(ModernFurnitureFactory);
    let m_chair = modern_factory.create_chair();
    let m_sofa = modern_factory.create_sofa();
    let m_table = modern_factory.create_coffee_table();
    
    m_chair.sit_on();
    m_sofa.lie_on();
    m_table.place_items();

    let victorian_factory: Box<dyn FurnitureFactory> = Box::new(VictorianFurnitureFactory);
    let v_chair = victorian_factory.create_chair();
    let v_sofa = victorian_factory.create_sofa();
    let v_table = victorian_factory.create_coffee_table();
    
    v_chair.sit_on();
    v_sofa.lie_on();
    v_table.place_items();
}
