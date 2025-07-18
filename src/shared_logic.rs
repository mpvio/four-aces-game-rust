use std::io;

use rand::seq::SliceRandom;

use crate::{cards::{CardsEnum, CardsStruct}, version_r_logic::version_r, version_y_logic::version_y};

pub fn game_start() {
    let play_version_y = choose_version();
    if play_version_y {
        version_y();
    } else {
        version_r();
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

pub fn prepare_deck(version_y: bool) -> Vec<CardsStruct> {
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

pub fn play_again() -> bool {
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