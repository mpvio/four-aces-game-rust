use crate::controllers::shared_logic::{game_start, game_start_ui};

pub mod models;
pub mod controllers;
pub mod view;

fn main() {
    rules();
    //game_start();
    game_start_ui();
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

















