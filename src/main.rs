mod utils;
use utils::*;
use utils::card_module::*;
use utils::deck_module::*;
use utils::mq_json_helpers::*;

use std::fs::File; // for file io
use std::fs::write; // for writing to a file
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
 //   let mut my_card = Card::new("Dark Magician", (100.0, 150.0), color_to_tuple(BLUE));
  //  let json = std::fs::read_to_string("my_set1.json").unwrap();
 //   let card_list = from_str::<Vec<Card>>(&json);
//    let card_list: Vec<Card> = serde_json::from_str(&json).expect("ERROR: issue deserializing JSON");
    let json = std::fs::read_to_string("src/Assets/test_read.json").expect("ERROR: issue reading json");
    let card_list: Vec<Card> = serde_json::from_str(&json).expect("ERROR: issue deserializing JSON");
    let mut my_deck = Deck::new(vec2(100.0, 150.0), GRAY);
    my_deck.set_deck(card_list);
    my_deck.print();

    let json_string = serde_json::to_string_pretty(&(my_deck.copy_deck())).expect("ERROR: issue serializing");
    write("src/Assets/test_write.json", json_string).expect("ERROR: issue writing json");
    
/*    
    loop {
        clear_background(WHITE);
        let (mouse_x, mouse_y) = mouse_position(); // every frame get the position of the mouse
        println!("mouse_x == {mouse_x}\nmouse_y == {mouse_y}");

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
    
   //     my_card.draw();
        
        next_frame().await
    }
*/
}