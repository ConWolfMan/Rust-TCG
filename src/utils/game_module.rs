use std::vec;

use macroquad::prelude::*;

use crate::utils::card_module::Card;
use crate::utils::deck_module::Deck;


// Manages the game field for both players
pub struct game_field{
    p1_hand: Option<Vec<Card>>,
    p1_deck: Option<Vec<Deck>>,

    p2_hand: Option<Vec<Card>>,
    p2_deck: Option<Vec<Deck>>,
}

impl game_field {

}