use std::fmt::Display;

#[derive(Debug)]
pub enum Winner {
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