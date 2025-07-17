pub enum Cards {
    AceSpade,
    AceClub,
    AceHeart,
    AceDiamond,
    Joker
}

impl Cards {
    pub fn color(&self) -> String {
        match self {
            Cards::AceSpade | Cards::AceClub => String::from("Black"),
            Cards::AceHeart | Cards::AceDiamond => String::from("Red"),
            Cards::Joker => String::from("Joker"),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn win_condition(a: Cards, b: Cards) -> bool {
    a.color() == b.color()
}
