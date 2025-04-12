use macroquad::prelude::*;
use serde::{Deserialize, Serialize}; // for serde/json features
use std::default::Default;

use crate::utils::mq_json_helpers::*;

// the following are made using these fields then changed later into those that macroquad expects, so that I can use json for file io
#[derive(Deserialize, Serialize, Debug)]
pub struct Card {
    #[serde(default = "default_name")]
    name: String,

//    #[serde(skip)]
    #[serde(default = "default_size")]
    size: (f32, f32),
//    #[serde(skip)]
    #[serde(default = "default_pos")]
    pos: (f32, f32),
/*
    "color": [1.0, 0.5, 0.0, 1.0] == ORANGE, for creature cards
    "color": [0.0, 1.0, 0.0, 1.0] == GREEN, for spell cards
    */
    #[serde(default = "default_color")]
    color: (f32, f32, f32, f32),
    
//    #[serde(skip)]
    #[serde(default = "default_dragging")]
    dragging: bool,
//    #[serde(skip)]
    #[serde(default = "default_drag_offset")]
    drag_offset: (f32, f32),
}

impl Card {
    pub fn new(new_name: &str, new_size: (f32, f32), new_color: (f32, f32, f32, f32)) -> Self {
        Self {
            name: new_name.to_string(),
            size: new_size,
            pos: (0.0, 0.0),
            color: new_color,
            dragging: false,
            drag_offset: (0.0, 0.0),
        }
    }

    pub fn draw_at(&mut self, new_pos: Vec2) {
        self.pos = vec2_to_tuple(new_pos);
        draw_rectangle(self.pos.0, self.pos.1, self.size.0, self.size.1, tuple_to_color(self.color));
        let font_size = 15.0;
        let text_width = measure_text(&self.name, None, font_size as _, 1.0).width;
        let self_pos = tuple_to_vec2(self.pos);
        let self_size = tuple_to_vec2(self.size);
        draw_text( // display the name of the card
            &self.name,
            self_pos.x + self_size.x / 2.0 - text_width / 2.0,
            self_pos.y + font_size,
            font_size,
            BLACK,
        );
    }

    pub fn draw(&mut self) {
        self.update();
        draw_rectangle(self.pos.0, self.pos.1, self.size.0, self.size.1, tuple_to_color(self.color));
        let font_size = 15.0;
        let text_width = measure_text(&self.name, None, font_size as _, 1.0).width;
        let self_pos = tuple_to_vec2(self.pos);
        let self_size = tuple_to_vec2(self.size);
        draw_text( // display the name of the card
            &self.name,
            self_pos.x + self_size.x / 2.0 - text_width / 2.0,
            self_pos.y + font_size,
            font_size,
            BLACK,
        );

    }

    fn contains(&self, pos: Vec2) -> bool {
        let self_pos: Vec2 = vec2(self.pos.0, self.pos.1);
        let self_size: Vec2 = vec2(self.size.0, self.size.1);

        pos.x >= self_pos.x
            && pos.x <= self_pos.x + self_size.x
            && pos.y >= self_pos.y
            && pos.y <= self_pos.y + self_size.y
    }

    fn update(&mut self) {
        let mouse_vec = vec2(mouse_position().0, mouse_position().1);

        if is_mouse_button_down(MouseButton::Left) && self.contains(mouse_vec) {
            self.dragging = true;
            self.drag_offset = vec2_to_tuple(mouse_vec - tuple_to_vec2(self.pos));
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.dragging = false;
        }

        if self.dragging {

            self.pos = vec2_to_tuple(mouse_vec - tuple_to_vec2(self.drag_offset));
        }
    }

    pub fn GetName(&self) -> String {
        self.name.clone()
    }

    pub fn clone(&self) -> Card {
        Card {
            name: self.name.clone(),
            size: self.size,
            pos: self.pos,
            color: self.color,
            dragging: false,
            drag_offset: (0.0, 0.0),
        }
    }
}

// name, size, pos, color, dragging, drag_offset

fn default_name() -> String {
    "Unnamed Card".to_string()
}

fn default_size() -> (f32, f32) {
    (100.0, 150.0)
}

fn default_pos() -> (f32, f32) {
    (0.0, 0.0)
}

fn default_color() -> (f32, f32, f32, f32) {
    (1.0, 1.0, 1.0, 1.0)
}

fn default_dragging() -> bool {
    false
}

fn default_drag_offset() -> (f32, f32) {
    (0.0, 0.0)
}

/*
impl Clone for Card {
    fn clone(&self) -> Card {
        Card {
            name: self.name.clone(),
            size: self.size,
            pos: self.pos,
            color: self.color,
            dragging: false,
            drag_offset: (0.0, 0.0),
        }
    }
}
*/

