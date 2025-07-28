use egui::{Button, CentralPanel, Color32, Id, Response, Sense, Stroke, Ui};
use rand::seq::IndexedRandom;

use crate::{controllers::shared_logic::prepare_deck, models::{cards::CardsStruct, winner::Winner}};


pub struct GameUI {
    deck: Option<Vec<CardsStruct>>,
    version_y: Option<bool>,
    // for version_y
    player_cards: Vec<CardsStruct>,
    opp_cards: Vec<CardsStruct>,
    player_turn: bool,
    winner: Winner
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            deck: None,
            version_y: None,
            player_cards: Vec::new(),
            opp_cards: Vec::new(),
            player_turn: true,
            winner: Winner::None
        }
    }
}

impl eframe::App for GameUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            display_game(
                ui, 
                &mut self.deck, 
                &mut self.version_y,
                &mut self.player_cards, 
                &mut self.opp_cards, 
                &mut self.player_turn, 
                &mut self.winner
            );
        });
    }
}

pub fn display_game(
    ui: &mut Ui,
    deck: &mut Option<Vec<CardsStruct>>,
    version_y: &mut Option<bool>,
    player_cards: &mut Vec<CardsStruct>,
    opp_cards: &mut Vec<CardsStruct>,
    player_turn: &mut bool,
    mut winner: &mut Winner
) {
    ui.vertical(|ui| {
        if version_y.is_none() {
            ui.label("Select Version:");
            // Store the temporary selection in a persistent way
            let mut version_y_temp = ui.memory(|mem| mem.data.get_temp::<Option<bool>>(Id::new("version_selection")).unwrap_or(None));
            ui.horizontal(|ui| {
                ui.radio_value(&mut version_y_temp, Some(false), "4 Card Version");
                ui.radio_value(&mut version_y_temp, Some(true), "5 Card Version");
            });
            ui.horizontal(|ui| {
                let confirm_button = ui.add_enabled(
                    version_y_temp.is_some(),
                    Button::new("Confirm")
                );

                if confirm_button.clicked() {
                    *version_y = version_y_temp;
                    *deck = Some(prepare_deck(version_y.unwrap()));
                    *player_turn = rand::random_bool(0.5);
                    // clear selection variable
                    ui.memory_mut(|mem| mem.data.insert_temp::<Option<bool>>(Id::new("version_selection"), None));
                }
            });
            // store selection for next frame
            ui.memory_mut(|mem| mem.data.insert_temp::<Option<bool>>(Id::new("version_selection"), version_y_temp));
        } else {
            // common code for 4 card & 5 card versions
            let version = version_y.unwrap();
            let num = if version {5} else {4};
            let max_card_selections = if version {1} else {2};
            let deck_ref = deck.as_mut().unwrap();
            ui.label(format!("{} Cards", num));

            if version {
                // 5 card unique logic
                ui.label(format!("Current turn: {}", if *player_turn { "Player" } else { "AI" }));

                let game_ended = five_card_game_over(player_cards, opp_cards, winner);

                // set up components
                let (selected_indices, confirm_button, restart_button) = display_card_selection(
                    ui, 
                    &deck_ref, 
                    max_card_selections, 
                    Id::new("selected_cards"), 
                    winner, 
                    *player_turn,
                    player_cards,
                    opp_cards
                );

                // only play if game is ongoing
                if !game_ended {
                    if *player_turn {
                        // human's turn
                        if confirm_button.clicked() && selected_indices.len() == 1 {
                            for &idx in &selected_indices {
                                deck_ref[idx].select();
                                player_cards.push(deck_ref[idx].clone());
                                // check if game ended, else change turn
                                if five_card_game_over(player_cards, opp_cards, winner) {
                                    break;
                                } else {
                                    *player_turn = false;
                                }
                            }

                            // clear selected_indices
                            ui.memory_mut(|mem| mem.data.insert_temp(Id::new("selected_cards"), Vec::<usize>::new()));
                            
                        }
                    } else {
                        // ai's turn
                        // get only the cards that haven't been selected yet
                        let available_cards: Vec<usize> = deck_ref.iter()
                            .enumerate()
                            .filter(|(_, card)| !card.is_selected())
                            .map(|(i, _)| i)
                            .collect();

                        if available_cards.len() > 1 {
                            // select a card if possible
                            let ai_choice = available_cards.choose(&mut rand::rng()).unwrap();
                            // get and process card
                            deck_ref[*ai_choice].select();
                            opp_cards.push(deck_ref[*ai_choice].clone());
                            // check if game is over, else change turn
                            if !five_card_game_over(player_cards, opp_cards, winner) {
                                *player_turn = true;
                            }
                        }
                    }
                }

                // restart game logic
                if restart_button.clicked() {
                    reset_game_state(
                        version_y,
                        deck,
                        player_cards,
                        opp_cards,
                        player_turn,
                        winner,
                        ui
                    );
                }

                // five card output
                ui.separator();
                ui.label(format!("Player 1's cards: {:?}", if player_cards.is_empty() { String::from("None") } else {CardsStruct::vec_to_string(&player_cards)}));
                ui.label(format!("Player 2's cards: {:?}", if opp_cards.is_empty() { String::from("None") } else {CardsStruct::vec_to_string(&opp_cards)}));
                match *winner {
                    Winner::Player1 => {ui.label("Player 1 wins!");},
                    Winner::Player2 => {ui.label("Player 2 wins!");},
                    Winner::None => {
                        // game is ongoing UNLESS four cards have been selected
                        if player_cards.len() == 2 && opp_cards.len() == 2 {
                            ui.label("DRAW!");
                        }
                    },
                }
            } else {
                // 4 card unique logic
                let allow_interaction = player_cards.len() != 2;
                let (selected_indices, confirm_button, restart_button) = display_card_selection(
                    ui,
                    &deck_ref, 
                    max_card_selections, 
                    Id::new("selected_cards"),
                    &mut winner,
                    allow_interaction,
                    player_cards,
                    opp_cards
                );

                // add player's cards to player_cards
                if confirm_button.clicked() {
                    for &idx in &selected_indices {
                        deck_ref[idx].select();
                        player_cards.push(deck_ref[idx].clone());
                    }
                }
               
                // restart game logic
                if restart_button.clicked() {
                    reset_game_state(
                        version_y,
                        deck,
                        player_cards,
                        opp_cards,
                        player_turn,
                        winner,
                        ui
                    );
                }

                // four aces output
                ui.separator();
                if player_cards.len() == 2 {
                    let result =  if CardsStruct::vec_has_pair(&player_cards) {
                        *winner = Winner::Player1;
                        String::from("win")
                    } else {
                        *winner = Winner::Player2;
                        String::from("lose")
                    };
                    ui.label(format!("You {}! Your cards: {:?}", result, CardsStruct::vec_to_string(player_cards)));
                } else {
                    ui.label("Your chosen cards will be displayed here.");
                }
            }
        }
        // insert instructions
        ui.separator();
        let _instructions = display_instructions(ui, version_y);
    });

}

fn display_card_selection(
    ui: &mut Ui,
    deck: &Vec<CardsStruct>,
    max_selections: usize,
    memory_id: Id,
    winner: &mut Winner,
    allow_interaction: bool,
    player_cards: &mut Vec<CardsStruct>,
    opp_cards: &mut Vec<CardsStruct>
) -> (Vec<usize>, Response, Response) {
    let mut selected_indices = ui.memory(|mem| 
        mem.data.get_temp::<Vec<usize>>(memory_id).unwrap_or_default()
    );

    ui.horizontal(|ui| {
        for (i, card) in deck.iter().enumerate() {
            let is_selected = card.is_selected() || selected_indices.contains(&i);
            let color = if is_selected { Color32::LIGHT_BLUE } else { Color32::DARK_GRAY };
            
            let card_panel = egui::Frame::default()
                .fill(color).stroke(Stroke::new(1.0, Color32::WHITE))
                .show(ui, |ui|{
                    if card.is_selected() {
                        ui.label(format!("{}", card));
                    } else {
                        ui.label("Unknown");
                    }
                }
            )
            .response;

            let two_card_limit_for_5_card_version = if max_selections == 2 {
                // ignore for 4 card version
                true
            } else {
                // else only allow card selections if player has less than 2 cards
                player_cards.len() < 2
            };

            let card_panel = if allow_interaction && *winner == Winner::None && !card.is_selected() && two_card_limit_for_5_card_version {
                card_panel.interact(Sense::click())
            } else {
                card_panel.interact(Sense::hover())
            };

            // panels are only clickable if there is no winner yet
            if card_panel.clicked() && allow_interaction && *winner == Winner::None {
                if is_selected {
                    // if clicking a selected card, it will be removed from indices (i.e. allowed to revert options before confirming)
                    selected_indices.retain(|&x| x != i);
                } else if selected_indices.len() < max_selections {
                    // adds i to index of selected cards
                    selected_indices.push(i);
                }
            }

            ui.add_space(5.0);
        }
    });

    let confirm_button = ui.add_enabled(
        selected_indices.len() == max_selections,
        Button::new("Confirm Selection")
    );

    // restart button logic
    let game_over = if max_selections == 2 {
        //4 cards version
        *winner != Winner::None
    } else {
        five_card_game_over(player_cards, opp_cards, winner)
    };

    let restart_button = ui.add_enabled(
        game_over,
        Button::new("Restart")
    );

    ui.memory_mut(|mem| mem.data.insert_temp(memory_id, selected_indices.clone()));

    (selected_indices, confirm_button, restart_button)
}

fn five_card_game_over(
    player_cards: &mut Vec<CardsStruct>,
    opp_cards: &mut Vec<CardsStruct>,
    winner: &mut Winner
) -> bool {
    if CardsStruct::vec_contains_joker(&player_cards) || CardsStruct::vec_has_pair(&opp_cards) {
        *winner = Winner::Player2; // player drew Joker OR opp has valid pair
        return true;
    } else if CardsStruct::vec_contains_joker(&opp_cards) || CardsStruct::vec_has_pair(&player_cards) {
        *winner = Winner::Player1; // opp drew Joker OR player has valid pair
        return true;
    } else if player_cards.len() == 2 && opp_cards.len() == 2 {
        *winner = Winner::None; // no cards left to draw
        return true;
    } else {
        return false;
    }
}

fn display_instructions(ui: &mut Ui, version_y: &mut Option<bool>) -> Response {
    let resp = ui.vertical(|ui| {
        match version_y {
            Some(version) => {
                if *version {
                    // 5 card rules
                        ui.label("Four Ace Cards and a Joker are laid face down before you.");
                        ui.label("You and your opponent take turns choosing one card each.");
                        ui.label("The winner is the first to pick two cards of the same color (Red/ Black).");
                        ui.label("However! Whoever picks the Joker loses immediately.");
                        ui.label("If there is only one card left to pick, the game ends in a DRAW.");
                } else {
                    // 4 card rules
                        ui.label("Four Ace Cards are laid before you, face down. Choose two of them at once.");
                        ui.label("You win if both are the same color (Red/ Black). Otherwise, you lose.");
                }
            },
            None => {
                    ui.label("Welcome!");
                    ui.label("Choose which version of the game you want to play.");
                    ui.label("Version specific rules will be displayed here once you do.");
            }
        }
        ui.label("Have fun!");
    }).response;
    return resp;
}

fn reset_game_state(
    version_y: &mut Option<bool>,
    deck: &mut Option<Vec<CardsStruct>>,
    player_cards: &mut Vec<CardsStruct>,
    opp_cards: &mut Vec<CardsStruct>,
    player_turn: &mut bool,
    winner: &mut Winner,
    ui: &mut Ui
) {
    // Reset game state
    *version_y = None;
    *deck = None;
    player_cards.clear();
    opp_cards.clear();
    *player_turn = true;
    *winner = Winner::None;
    
    // Clear UI memory
    ui.memory_mut(|mem| {
        mem.data.remove_temp::<Option<bool>>(Id::new("version_selection"));
        mem.data.remove_temp::<Vec<usize>>(Id::new("selected_cards"));
    });
}