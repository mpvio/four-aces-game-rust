use std::io;
use rand::seq::SliceRandom;
use std::collections::HashSet;

use crate::cards::Cards;

pub mod cards;

fn main() {
    println!("Hello, world!");
    version_r();
}

fn version_r() {
    let mut deck = prepare_deck(false);
    let chosen = select_two_cards(&mut deck);
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


fn select_two_cards(deck: &mut Vec<Cards>) -> Vec<&Cards> {
  let mut buffer = String::new();
  let mut selections: Vec<&Cards> = vec![];
  let mut input_set: HashSet<usize> = HashSet::new();
  // init attempt to get inputs
  println!("Enter 2 non-identical numbers from 1 to {}, separated by a space:", deck.len());
  let stdin = io::stdin();
  let result = match stdin.read_line(&mut buffer) {
    Ok(_) => {
        buffer.trim().to_string()
    },
    Err(_) => String::new(),
  };
  let ls = result.split_ascii_whitespace().collect::<HashSet<&str>>();
  for val in ls {
    if let Ok(num) = val.parse::<usize>() {
        //print!("{num}: ");
        if num >= 1 && num <= deck.len() {
            input_set.insert(num);
            //println!("added.")
        } else {
            //println!("refused.")
        }
    }
  }
  // less than 2 valid values entered
  while input_set.len() < 2 {
    println!("Enter {} non-identical numbers from 1 to {}, separated by a space. Currently selected: {:?}", 2-input_set.len(), deck.len(), input_set);
    buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(_) => {
                buffer.trim().to_string()
            },
            Err(_) => String::new(),
        };
        for val in buffer.trim().split_ascii_whitespace() {
            if let Ok(num) = val.parse::<usize>() {
                //print!("{num}: ");
                if num >= 1 && num <= deck.len() {
                    input_set.insert(num);
                    //println!("added.")
                }else {
                    //println!("refused.")
                }
            }
        }
  }
  if input_set.len() > 2 {
    println!("Only first two values will be accepted.")
  }
  // convert numbers to cards
  for val in input_set {
    if let Some(card) = deck.get(val-1) {
        selections.push(card);
    }
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
