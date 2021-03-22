use std::collections::HashMap;
use nannou::prelude::*;

mod l_system;
pub use crate::l_system::produce;
mod turtle;
pub use crate::turtle::Turtle;

fn to_rad(deg: f32) -> f32 {
    PI/180.0*deg
}  

pub fn render_turtle(draw: &Draw, path: &str, mut turtle: Turtle) {
    let scaling = 1.0;
    let turning_angle_increment = to_rad(5.0);
    let line_length_scaling_factor = 1.5;

    for c in path.chars() {
        match c.to_string().as_str() {
            // Move forward by line length drawing a line
            "F" => {
                turtle.forward(draw, turtle.line_length * scaling);
            }
            // Move forward by line length without drawing a line
            "f" => {
                turtle.forward_no_draw(turtle.line_length * scaling);
            }
            // Turn left by turning angle
            "+" => {
                turtle.turn(-turtle.turn_angle);
            }
            // Turn right by turning angle
            "-" => {
                turtle.turn(turtle.turn_angle);
            }
            // Reverse direction (ie: turn by 180 degrees)
            "|" => {
                turtle.turn(to_rad(180.0));
            }
            // Push current drawing state onto stack
            "[" => {
                turtle.push();
            }
            // Pop current drawing state from the stack
            "]" => {
                turtle.pop();
            }
            // Increment the line width by line width increment
            "#" => {
                turtle.increment_thickness(turtle.thickness);
            }
            // Decrement the line width by line width increment
            "!" => {
                turtle.decrement_thickness(turtle.thickness);
            }
            // Draw a dot with line width radius
            "@" => {
                turtle.dot(draw, turtle.thickness);
            }
            // Open a polygon
            "{" => {}
            // Close a polygon and fill it with fill colour
            "}" => {}
            // Swap the meaning of + and -
            "&" => {
                turtle.reverse_turn();
            }
            // Decrement turning angle by turning angle increment
            "(" => {
                turtle.turn_angle -= turning_angle_increment;
            }
            // Increment turning angle by turning angle increment
            ")" => {
                turtle.turn_angle += turning_angle_increment;
            }
            // Multiply the line length by the line length scale factor
            ">" => {
                turtle.line_length *= line_length_scaling_factor;
            }	
            // Divide the line length by the line length scale factor
            "<" => {
                turtle.line_length /= line_length_scaling_factor;
            } 
            // X does not correspond to any drawing action and is used to control the evolution of the curve. 
            "X" => {}
            
            _ => {
                println!("unknown command")
            }
        }
    }
}

struct Model {
    axiom: String,
    production: String,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap();

    let axiom = String::from("X");
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("X"), String::from("F+[[X]-X]-F[-FX]+X"));
    production_rules.insert(String::from("F"), String::from("FF"));

    let mut production = axiom.clone();
    for _ in 0..6 {
        production = produce(&production, &production_rules)
    }

    Model { 
        axiom: axiom,
        production: production,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(WHITE);

    let turtle = Turtle{
        position: vec2(
            app.window_rect().mid_bottom().x, 
            app.window_rect().bottom()),
        orientation: 0.0,
        thickness: 2.0,
        color: FORESTGREEN,
        stack: Vec::new(),
        turn_angle: to_rad(25.0),
        turn_reversed: false,
        line_length: 3.0,
    };

    render_turtle(&draw, &model.production, turtle);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}