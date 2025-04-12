use macroquad::prelude::*;

use std::default::Default;

use crate::utils::card_module::Card;

pub struct Deck {
    deck: Vec<Card>,

    size: Vec2,
    pos: Vec2,   
    color: Color,
}


impl Deck {
    pub fn new(new_size: Vec2, new_color: Color) -> Self {
        Self {
            deck: Vec::new(),
            size: new_size,
            pos: vec2(0.0, 0.0),
            color: new_color,
        }
    }

    pub fn load_card(&mut self,new_card: Card) {  
               self.deck.push(new_card);
    }

    // draws a card off the top of the deck, returning the top card
    pub fn draw_card(&mut self) -> Option<Card> {
        self.deck.pop()   
    }   

    // this is the normal, draw function
    pub fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, self.color);
    }

    pub fn draw_at(&self, new_pos: Vec2) {
        draw_rectangle(new_pos.x, new_pos.y, self.size.x, self.size.y, self.color);
    }

    pub fn set_deck(&mut self, new_deck: Vec<Card>) {
        self.deck = new_deck;        
    }

    pub fn print(&self) {
        for card in self.deck.iter().clone() {
            println!("{}", card.GetName());
        }
    }

    pub fn copy_deck(&self) -> Vec<Card> {
        let mut new_deck = Vec::new();
        for card in self.deck.iter().clone() {
            new_deck.push(card.clone());
        }
        new_deck
    }

}




