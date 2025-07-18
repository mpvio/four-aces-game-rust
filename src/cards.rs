use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum CardsEnum {
    AceSpade,
    AceClub,
    AceHeart,
    AceDiamond,
    Joker
}

impl CardsEnum {
    pub fn color(&self) -> String {
        match self {
            CardsEnum::AceSpade | CardsEnum::AceClub => String::from("Black"),
            CardsEnum::AceHeart | CardsEnum::AceDiamond => String::from("Red"),
            CardsEnum::Joker => String::from("Joker"),
        }
    }
}

impl Display for CardsEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let val = match self {
            CardsEnum::AceSpade => String::from("Spade"),
            CardsEnum::AceClub => String::from("Club"),
            CardsEnum::AceHeart => String::from("Heart"),
            CardsEnum::AceDiamond => String::from("Diamond"),
            CardsEnum::Joker => String::from("Joker"),
        };
        write!(f, "{} ({})", val, self.color())
    }
}

#[derive(Debug)]
pub struct CardsStruct {
    pub suit: CardsEnum,
    pub selected: bool
}

impl CardsStruct {
    pub fn new(suit: CardsEnum) -> Self {
        Self {
            suit,
            selected: false
        }
    }

    pub fn color(&self) -> String {
        self.suit.color()
    }
}

impl Display for CardsStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.suit)
    }
}