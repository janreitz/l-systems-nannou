use std::ops::Mul;

// use std::collections::HashMap;
use nannou::{math::{Deg, Euler, Quaternion}, prelude::*};

pub struct Turtle {
    pub position: Vector3,
    pub orientation: Vector3, // around x, y, z
    pub thickness: f32,
    pub color: Rgb8,
    pub stack: Vec<(Vector3, Vector3)>,
    pub turn_reversed: bool,
    pub turn_angle: f32,
    pub line_length: f32,
}

impl Default for Turtle {
    fn default() -> Turtle {
        Turtle{
            position: vec3(0.0, 0.0, 0.0),
            orientation: Vector3::unit_y(),
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
        let new_position = self.position + self.orientation.mul(dist);
        
        draw.line()
            .start(vec2(self.position.x, self.position.y))
            .end(vec2(new_position.x, new_position.y))
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
        self.position += self.orientation.mul(dist);
    }

    pub fn turn(& mut self, mut deg: f32) {
        if self.turn_reversed {
            deg = -deg;
        }
        self.rotate(0.0, 0.0, deg);
    }
    
    pub fn turn_x(& mut self, mut deg: f32) {
        if self.turn_reversed {
            deg = -deg;
        }
        self.rotate(deg, 0.0, 0.0);
    }
    
    pub fn turn_y(& mut self, mut deg: f32) {
        if self.turn_reversed {
            deg = -deg;
        }
        self.rotate(0.0, deg, 0.0);
    }

    fn rotate(& mut self, x: f32, y: f32, z: f32)
    {
        let rotation = Quaternion::from(Euler {
            x: Deg(x),
            y: Deg(y),
            z: Deg(z),
        });

        self.orientation = rotation.rotate_vector(self.orientation.into()).into();
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
