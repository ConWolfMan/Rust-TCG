use macroquad::prelude::*;

/* the Json library for rust does not have built in features for macroquad functanality
so to work around this, I have made helpful tools to translated between what json can work with, and what macroquad can work with
*/

pub fn color_to_tuple(my_color: Color) -> (f32, f32, f32, f32) {
    (my_color.r, my_color.g, my_color.b, my_color.a)
}

pub fn tuple_to_color(my_tuple: (f32, f32, f32, f32)) -> Color {
    Color::new(my_tuple.0, my_tuple.1, my_tuple.2, my_tuple.3)
}

pub fn vec2_to_tuple(my_vec: Vec2) -> (f32, f32) {
    (my_vec.x, my_vec.y)
}

pub fn tuple_to_vec2(my_tuple: (f32, f32)) -> Vec2 {
    vec2(my_tuple.0, my_tuple.1)
}