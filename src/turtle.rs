use std::ops::Mul;

// use std::collections::HashMap;
use nannou::{
    math::{
        Matrix3, 
        Deg, 
        cgmath::Vector3,
    }, 
    prelude::*
};

pub struct Turtle {
    pub position: Vector3<f32>,
    pub orientation: Matrix3<f32>,
    pub thickness: f32,
    pub color: Rgb8,
    pub stack: Vec<(Vector3<f32>, Matrix3<f32>)>,
    pub turn_reversed: bool,
    pub turn_angle: Deg<f32>,
    pub line_length: f32,
}

impl Default for Turtle {
    fn default() -> Turtle {
        Turtle{
            position: vec3(0.0, 0.0, 0.0).into(),
            orientation: Matrix3::from_cols(
                Vector3::unit_x().into(),
                Vector3::unit_y().into(),
                Vector3::unit_z().into()
            ),
            thickness: 2.0,
            color: FORESTGREEN,
            stack: Vec::new(),
            turn_angle: Deg(25.0),
            turn_reversed: false,
            line_length: 1.0,
        }
    }
}

impl Turtle {
    pub fn forward(& mut self, draw: &Draw, dist: f32) {
        let new_position = self.position + self.orientation.x.mul(dist);
        
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
        self.position += self.orientation.x.mul(dist);
    }
    

    pub fn yaw(& mut self, mut deg: Deg<f32>) {
        if self.turn_reversed {
            deg = -deg;
        }

        let rotation = Matrix3::from_axis_angle(
            self.orientation.z,
            deg
        );

        self.orientation = rotation * self.orientation;
    }
    
    pub fn roll(& mut self, mut deg: Deg<f32>) {
        if self.turn_reversed {
            deg = -deg;
        }
        let rotation = Matrix3::from_axis_angle(
            self.orientation.x,
            deg
        );
    
        self.orientation = rotation * self.orientation;
    }
    
    pub fn pitch(& mut self, mut deg: Deg<f32>) {
        if self.turn_reversed {
            deg = -deg;
        }
        let rotation = Matrix3::from_axis_angle(
            self.orientation.y,
            deg
        );
    
        self.orientation = rotation * self.orientation;
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
