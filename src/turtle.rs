// use std::collections::HashMap;
use nannou::prelude::*;

pub struct Turtle {
    pub position: Vector2,
    pub orientation: f32, // 12:00 -> 0 03:00 -> PI/4
    pub thickness: f32,
    pub color: Rgb8,
    pub stack: Vec<(Vector2, f32)>
}

impl Turtle {
    pub fn forward(& mut self, draw: &Draw, dist: f32) {
        let new_position = self.position + vec2(
            dist * self.orientation.sin(),
            dist * self.orientation.cos(),
        );

        draw.line()
            .start(self.position)
            .end(new_position)
            .weight(self.thickness)
            .color(self.color);

        self.position = new_position;
    }

    pub fn turn(& mut self, rad: f32) {
        self.orientation = self.orientation + rad;
    }

    pub fn push(&mut self) {
        self.stack.push((self.position, self.orientation));
    }

    pub fn pop(&mut self) {
        match self.stack.pop() {
            Some(popped) => {
                let (position, orientation) = popped;
                self.position = position;
                self.orientation = orientation;
            }
            None => println!("Popped off empty stack")
        }
    }
}
