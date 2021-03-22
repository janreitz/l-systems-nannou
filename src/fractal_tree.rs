use std::collections::HashMap;
use nannou::prelude::*;

mod l_system;
pub use crate::l_system::produce;
mod turtle;
pub use crate::turtle::Turtle;

pub fn render_turtle(draw: &Draw, path: &str) {
    let mut turtle = Turtle{
        position: vec2(0.0, -512.0),
        orientation: 0.0,
        thickness: 5.0,
        color: STEELBLUE,
        .. Turtle::default()
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