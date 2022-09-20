#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&mut self, color: Option<ShirtColor>) -> ShirtColor {
        color.unwrap_or_else(|| self.most_available())
    }

    fn most_available(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let mut inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    let pref = Some(ShirtColor::Blue);
    let giveaway = inventory.giveaway(pref);
    println!("user-1 with preference {:?} got {:?}", pref, giveaway);

    let pref = None;
    let giveaway = inventory.giveaway(pref);
    println!("user-2 with preference {:?} got {:?}", pref, giveaway);
}
