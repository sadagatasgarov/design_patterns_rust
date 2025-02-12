

// #[derive(Debug, Clone)]
// struct Monster {
//     health: u32,
//     attack: u32,
// }


//Örnek: Elle Kopyalama
#[derive(Debug)]
struct Monster {
    health: u32,
    attack: u32,
}

impl Monster {
    // Kendi kopyalama fonksiyonumuzu yazıyoruz
    fn copy(&self) -> Monster {
        Monster {
            health: self.health,
            attack: self.attack,
        }
    }
}


// Daha Karmaşık Bir Yapı İçin Elle Kopyalama
#[derive(Debug)]
struct Robot {
    name: String,
    abilities: Vec<String>,
}

impl Robot {
    fn copy(&self) -> Robot {
        Robot {
            name: self.name.clone(), // String kopyalama
            abilities: self.abilities.clone(), // Vec kopyalama
        }
    }
}

fn main() {

    let prototype = Monster { health: 100, attack: 20 };

    // Prototype'den yeni nesneler türetiyoruz
    let mut enemy1 = prototype.copy();
    enemy1.health = 80; // Yeni düşmana özel değişiklik

    let mut enemy2 = prototype.copy();
    enemy2.attack = 30; // Farklı bir değişiklik

    println!("{:?}", prototype); // Orijinal değişmez
    println!("{:?}", enemy1);    // Sağlık 80 oldu
    println!("{:?}", enemy2);    // Saldırı 30 oldu






    //içinde Vec<String> veya başka yapılar olan bir struct

    let prototype = Robot {
        name: "T-800".to_string(),
        abilities: vec!["Shoot".to_string(), "Scan".to_string()],
    };

    let mut robot1 = prototype.copy();
    robot1.name = "T-1000".to_string(); // İsmi değiştirdik

    let mut robot2 = prototype.copy();
    robot2.abilities.push("Fly".to_string()); // Yeteneğe yeni bir özellik ekledik

    println!("{:?}", prototype); // Orijinal değişmedi
    println!("{:?}", robot1);    // İsmi değişti
    println!("{:?}", robot2);    // Yeteneği değişti
}
