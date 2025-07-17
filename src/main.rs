use std::io;
use rand::seq::SliceRandom;

#[derive(Debug)]
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
    version_r();
}

fn version_r() {
    let mut deck = prepare_deck(false);
    let chosen = select_cards(&mut deck);
    match (chosen.first(), chosen.last()) {
        (Some(a), Some(b)) => {
            println!("{}: {:#?} & {:#?}", win_condition(a, b), a, b);
        },
        (_, _) => {}
    }
    if play_again() {
        version_r();
    };
}

fn prepare_deck(version_y: bool) -> Vec<Cards> {
    let mut cards = vec![Cards::AceSpade, Cards::AceClub, Cards::AceHeart, Cards::AceDiamond];
    if version_y {
        cards.push(Cards::Joker);
    }
    let mut rng = rand::rng();
    cards.shuffle(&mut rng);
    return cards;
}

fn select_cards(deck: &mut Vec<Cards>) -> Vec<&Cards> {
  let mut buffer = String::new();
  let mut selections: Vec<&Cards> = vec![];
  println!("Enter two numbers from 1 to {}, separated by a space:", deck.len());
  let stdin = io::stdin();
  let result = match stdin.read_line(&mut buffer) {
    Ok(_) => {
        buffer.trim().to_string()
    },
    Err(_) => String::new(),
  };
  let ls = result.split_ascii_whitespace();
  for val in ls.into_iter() {
    match val.parse::<usize>() {
        Ok(num) => {
            if let Some(card) = deck.get(num-1) {
                selections.push(card);
            };
        },
        Err(_) => {

        },
    };
  }
  selections
  
}

fn play_again() -> bool {
    let mut buffer = String::new();
    println!("Play again? Y/N:");
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => {
            let answer = buffer.trim().to_string();
            answer.contains("Y") || answer.contains("y")
        },
        Err(_) => {
            play_again()
        },
    }
}

fn win_condition(a: &Cards, b: &Cards) -> bool {
    a.color() == b.color()
}
