use std::collections::HashMap;
use nannou::prelude::*;
use nannou::math::Deg;

mod l_system;
pub use crate::l_system::produce;
mod turtle;
pub use crate::turtle::Turtle;

fn to_rad(deg: f32) -> f32 {
    PI/180.0*deg
}  

pub fn render_turtle(draw: &Draw, path: &str) {
    let mut turtle = Turtle{
        position: vec3(0.0, -512.0, 0.0).into(),
        thickness: 2.0,
        .. Turtle::default()
    };

    let scaling = 1.0;

    for c in path.chars() {
        match c.to_string().as_str() {
            // F means "draw forward"
            "F" => {
                turtle.forward(draw, 5.0 * scaling);
            }
            // − means "turn right 25°"
            "-" => {
                turtle.yaw(Deg(25.0));
            }
            // + means "turn left 25°"
            "+" => {
                turtle.yaw(Deg(-25.0));
            }
            // X does not correspond to any drawing action and is used to control the evolution of the curve. 
            "X" => {}
            // [ save current values for position and angle
            "[" => {
                turtle.push();
            }
            // ]: pop position and angle
            "]" => {
                turtle.pop();
            }
            _ => {
                println!("unknown command")
            }
        }
    }
}

struct Model {
    path: String,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap();

    let mut axiom = String::from("X");
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("X"), String::from("F+[[X]-X]-F[-FX]+X"));
    production_rules.insert(String::from("F"), String::from("FF"));

    for _ in 0..6 {
        axiom = produce(&axiom, &production_rules)
    }

    Model { 
        path: axiom,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(WHITE);

    let mut path_substring = String::new();

    for (i, c) in model.path.chars().enumerate() {
        if (i as u64) >= app.elapsed_frames() * 8 {
            break
        }
        path_substring = path_substring + &c.to_string();
    }

    render_turtle(&draw, &path_substring);
    draw.text(&app.fps().to_string()).x_y(-500.0, 500.0);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}