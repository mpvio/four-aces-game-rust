use std::io;

use rand::seq::SliceRandom;

use crate::{controllers::{version_r_logic::version_r, version_y_logic::version_y}, models::cards::{CardsEnum, CardsStruct}, view::game_ui::GameUI};

pub fn game_start_ui() {
    let app = GameUI::new();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Four Aces", 
        options, 
        Box::new(|_cc| Ok(Box::new(app)))
    ).unwrap();
}

pub fn game_start_non_ui() {
    rules();
    let play_version_y = choose_version();
    if play_version_y {
        version_y();
    } else {
        version_r();
    }
}

fn rules() {
    println!("Welcome!");
    println!("Rules:");
    println!("");
    println!("Version R:");
    println!("The Four Ace Cards are laid before you, face down. Choose two of them at once.");
    println!("You win if both are the same color (Red/ Black). Otherwise, you lose.");
    println!("");
    println!("Version Y:");
    println!("Four Ace Cards and a Joker are laid face down before you.");
    println!("You and your opponent take turns choosing one card each.");
    println!("The winner is the first to pick two cards of the same color (Red/ Black).");
    println!("However! Whoever picks the Joker loses immediately.");
    println!("If there is only one card left to pick, the game ends in a DRAW.");
    println!("");
    println!("Have fun!");
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