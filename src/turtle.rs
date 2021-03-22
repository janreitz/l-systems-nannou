// use std::collections::HashMap;
use nannou::prelude::*;

pub struct Turtle {
    pub position: Vector2,
    pub orientation: f32, // 12:00 -> 0 03:00 -> PI/4
    pub thickness: f32,
    pub color: Rgb8,
    pub stack: Vec<(Vector2, f32)>,
    pub turn_reversed: bool,
    pub turn_angle: f32,
    pub line_length: f32,
}

impl Default for Turtle {
    fn default() -> Turtle {
        Turtle{
            position: vec2(0.0, -512.0),
            orientation: 0.0,
            thickness: 2.0,
            color: FORESTGREEN,
            stack: Vec::new(),
            turn_angle: PI/180.0*25.0,
            turn_reversed: false,
            line_length: 1.0,
        }
    }
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

    pub fn dot(& mut self, draw: &Draw, radius: f32) {
        draw.ellipse()
        .radius(radius)
        .color(self.color);
    }

    pub fn forward_no_draw(& mut self, dist: f32) {
        let new_position = self.position + vec2(
            dist * self.orientation.sin(),
            dist * self.orientation.cos(),
        );

        self.position = new_position;
    }

    pub fn turn(& mut self, rad: f32) {
        if self.turn_reversed {
            self.orientation = self.orientation - rad;
        } else {
            self.orientation = self.orientation + rad;
        }
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

    pub fn increment_thickness(&mut self, increment: f32) {
        self.thickness += increment;
    }

    pub fn decrement_thickness(&mut self, decrement: f32) {
        self.thickness -= decrement;
    }

    pub fn reverse_turn(&mut self) {
        self.turn_reversed = !self.turn_reversed;
    }
}
