use std::{collections::HashSet, fmt::{Display, Formatter, Result}};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn is_joker(&self) -> bool {
        self.suit.color() == String::from("Joker")
    }

    pub fn vec_contains_joker(cards: &Vec<CardsStruct>) -> bool {
        cards.iter().any(|card| card.is_joker())
    }

    pub fn vec_has_pair(cards: &Vec<CardsStruct>) -> bool {
        /* 
        for each elem in cards:
        - create a hashset (no duplicates) containing their color values
        - hashset.len < cards.len only if colors match
        */
        cards.iter()
        .map(|card| card.color())
        .collect::<HashSet<String>>()
        .len() < cards.len()
    }
}

impl Display for CardsStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.suit)
    }
}