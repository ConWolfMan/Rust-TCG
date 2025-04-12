use macroquad::prelude::*;


pub struct button {
    size: (f32, f32),
    pos: (f32, f32),
}

impl button {

    

    fn contains(&self, pos: Vec2) -> bool {
        let self_pos: Vec2 = vec2(self.pos.0, self.pos.1);
        let self_size: Vec2 = vec2(self.size.0, self.size.1);

        pos.x >= self_pos.x
            && pos.x <= self_pos.x + self_size.x
            && pos.y >= self_pos.y
            && pos.y <= self_pos.y + self_size.y
    }
}