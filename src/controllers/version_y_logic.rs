use std::io;

use rand::seq::IndexedRandom;

use crate::{models::cards::CardsStruct, controllers::shared_logic::{game_start_non_ui, play_again, prepare_deck}, models::winner::Winner};

pub fn version_y() {
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
        game_start_non_ui();
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