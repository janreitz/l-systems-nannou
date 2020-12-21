use std::collections::HashMap;
use nannou::prelude::*;

mod l_system;
pub use crate::l_system::produce;

struct Turtle {
    position: Vector2,
    orientation: f32, // 12:00 -> 0 03:00 -> PI/4
    thickness: f32,
    color: Rgb8,
    stack: Vec<(Vector2, f32)>
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

pub fn render_turtle(draw: &Draw, path: &str) {
    let mut turtle = Turtle{
        position: vec2(0.0, -512.0),
        orientation: 0.0,
        thickness: 5.0,
        color: STEELBLUE,
        stack: Vec::new(),
    };

    let scaling = 10.0/(path.len() as f32).sqrt();

    for c in path.chars() {
        match c.to_string().as_str() {
            // 0: draw a line segment ending in a leaf
            "0" => {
                turtle.color = GREEN;
                turtle.forward(draw, 5.0 * scaling);
            }
            // 1: draw a line segment
            "1" => {
                turtle.color = BROWN;
                turtle.forward(draw, 10.0 * scaling);
            }
            // [: push position and angle, turn left 45 degrees
            "[" => {
                turtle.push();
                turtle.turn(-PI/8.0);
            }
            // ]: pop position and angle, turn right 45 degrees
            "]" => {
                turtle.pop();
                turtle.turn(PI/8.0);
            }
            _ => {
                println!("unknown command")
            }
        }
    }
}

struct Model {
    path: String,
    rules: HashMap<String, String>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap();

    let axiom = String::from("0");
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("0"), String::from("1[0]0"));
    production_rules.insert(String::from("1"), String::from("11"));

    Model { 
        path: axiom,
        rules: production_rules,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() % 60 == 0{
        model.path = produce(&model.path, &model.rules)
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(BLACK);
    println!("Rendering path '{}", model.path);
    render_turtle(&draw, &model.path);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}