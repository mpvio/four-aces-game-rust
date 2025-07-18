use std::{fmt::Display, io};
use rand::seq::{IndexedRandom, SliceRandom};
use std::collections::HashSet;

use crate::cards::{CardsEnum, CardsStruct};

pub mod cards;

fn main() {
    println!("Welcome!");
    game_start();
}

fn game_start() {
    let play_version_y = choose_version();
    if play_version_y {
        version_y();
    } else {
        version_r();
    }
}

#[derive(Debug)]
enum Winner {
    Player1,
    Player2,
    None
}

impl Display for Winner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Winner::Player1 => write!(f, "Player 1 wins!"),
            Winner::Player2 => write!(f, "Player 2 wins!"),
            Winner::None => write!(f, "Draw!"),
        }
    }
}

fn choose_version() -> bool {
    let mut buffer = String::new();
    println!("Play version R or Y? (Enter R or Y):");
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => {
            let answer = buffer.trim().to_string();
            let version_r = answer.contains("r") || answer.contains("R");
            let version_y = answer.contains("y") || answer.contains("Y");
            if version_r {
                false
            } else if version_y {
                true
            } else {
                choose_version()
            }
        },
        Err(_) => {
            choose_version()
        },
    }
}

fn version_r() {
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

fn version_y() {
    let mut deck = prepare_deck(true);
    let mut player_cards = Vec::<CardsStruct>::new();
    let mut opp_cards = Vec::<CardsStruct>::new();
    let mut invalid_numbers = Vec::<usize>::new();

    // determine play order
    let player_first = player_first();
    let mut player_turn = player_first;
    let mut winner = Winner::None;
    loop {
        if invalid_numbers.len() > 3 {
            // no point playing if only one card is left
            // i.e. four cards of five have been selected right now
            break;
        } else {
            let some_card = select_one_card(&mut deck, &mut invalid_numbers, player_turn);
            if let Some(card) = some_card {
                let card_clone = card.clone();
                if player_turn {
                    player_cards.push(card_clone);
                    // winning pair?
                    if CardsStruct::vec_has_pair(&player_cards) {
                        winner = Winner::Player1;
                        break;
                    }
                } else {
                    opp_cards.push(card_clone);
                    // winning pair?
                    if CardsStruct::vec_has_pair(&opp_cards) {
                        winner = Winner::Player2;
                        break;
                    }
                }
                // drawing a joker ends the game immediately
                if card.is_joker() {
                    winner = if player_turn {
                        Winner::Player2
                    } else {
                        Winner::Player1
                    };
                    break;
                }
            }
            // change turn
            player_turn = !player_turn;
        }
    }

    println!("{}", winner);
    print!("Player 1: ");
    print_cards(&player_cards);
    print!("Player 2: ");
    print_cards(&opp_cards);

    if play_again() {
        game_start();
    }

}

fn player_first() -> bool {
    rand::random_bool(0.5)
}

fn print_cards(cards: &Vec<CardsStruct>) {
    for card in cards {
        print!("{} ", card);
    }
    println!("");
}

fn select_one_card(deck: &mut Vec<CardsStruct>, invalid_numbers: &mut Vec<usize>, player: bool) -> Option<CardsStruct> {
    let mut selection: usize = 0;
    while selection == 0 {
        if player {
            // keep asking player until a valid number is obtained
            let mut buffer = String::new();
            println!("Enter 1 number from 1 to {}. Must not be in {:?}:", deck.len(), invalid_numbers);
            let stdin = io::stdin();
            if let Ok(_) = stdin.read_line(&mut buffer) {
                if let Some(val) = buffer.trim().split_ascii_whitespace().collect::<Vec<&str>>().first() {
                    if let Ok(num) = val.parse::<usize>() {
                        if num >= 1 && num <= deck.len() && !invalid_numbers.contains(&num) {
                            selection = num;
                        }
                    }
                }
            }
        } else {
            // get a valid number for ai
            let valid_indices: Vec<usize> = (1..=deck.len())
                .filter(|num| !invalid_numbers.contains(num))
                .collect();
            if !valid_indices.is_empty() {
                selection = *valid_indices.choose(&mut rand::rng()).unwrap();
            } else {
                return None;
            }
            // let range = 1..deck.len()+1;
            // let valid_numbers: Vec<usize> = range.filter(|num| !invalid_numbers.contains(num)).collect();
            // selection = rand::random_range(1..valid_numbers.len()+1);
        }
    }
    invalid_numbers.push(selection);
    deck.get(selection-1).cloned()
    // if let Some(card) = deck.get(selection-1) {
    //     invalid_numbers.push(selection);
    //     return Some(card.clone()); 
    // };
    // None
}

fn prepare_deck(version_y: bool) -> Vec<CardsStruct> {
    let mut cards = vec![
        CardsStruct::new(CardsEnum::AceClub),
        CardsStruct::new(CardsEnum::AceDiamond),
        CardsStruct::new(CardsEnum::AceHeart),
        CardsStruct::new(CardsEnum::AceSpade)
    ];
    if version_y {
        cards.push(CardsStruct::new(CardsEnum::Joker));
    }
    let mut rng = rand::rng();
    cards.shuffle(&mut rng);
    return cards;
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

fn win_condition(a: &CardsStruct, b: &CardsStruct) -> String {
    if a.color() == b.color() {
        String::from("You Win!")
    } else {
        String::from("You Lose!")
    }
}
