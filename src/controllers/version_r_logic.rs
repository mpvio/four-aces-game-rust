use std::{collections::HashSet, io};

use crate::{models::cards::CardsStruct, controllers::shared_logic::{game_start, play_again, prepare_deck}};

pub fn version_r() {
    let mut deck = prepare_deck(false);
    let chosen = select_two_cards(&mut deck);
    match (chosen.first(), chosen.last()) {
        (Some(a), Some(b)) => {
            println!("{}: {} & {}", win_condition(a, b), a, b);
        },
        (_, _) => {}
    }
    if play_again() {
        game_start();
    };
}

fn select_two_cards(deck: &mut Vec<CardsStruct>) -> Vec<&CardsStruct> {
  let mut buffer = String::new();
  let mut selections: Vec<&CardsStruct> = vec![];
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

fn win_condition(a: &CardsStruct, b: &CardsStruct) -> String {
    if a.color() == b.color() {
        String::from("You Win!")
    } else {
        String::from("You Lose!")
    }
}