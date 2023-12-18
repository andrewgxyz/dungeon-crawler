use crate::prelude::*;

pub struct Player {
    pub position: Point
}

impl Player {
    pub fn new (position: Point) -> Self {
        Self {
            position
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@')
        );
    }

    pub fn update (&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
    }
}
