use std::collections::HashMap;
use nannou::prelude::*;

mod l_system;
pub use crate::l_system::produce_stochastic;
mod turtle;
pub use crate::turtle::Turtle;

fn to_rad(deg: f32) -> f32 {
    PI/180.0*deg
}  

pub fn render_turtle(draw: &Draw, path: &str) {
    let mut turtle = Turtle{
        position: vec2(0.0, -512.0),
        orientation: 0.0,
        thickness: 2.0,
        color: FORESTGREEN,
        stack: Vec::new(),
    };

    let scaling = 1.0;

    for c in path.chars() {
        match c.to_string().as_str() {
            "a" => {
                turtle.forward(draw, 50.0 * scaling);
            }
            "b" => {
                turtle.turn(to_rad(25.0));
            }
            "c" => {
                turtle.turn(to_rad(-25.0));
            }
            "d" => {
                // do nothing
            }
            "[" => {
                turtle.push();
            }
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
    axiom: String,
    path: String,
    iteration_count: i32,
    rules: HashMap<String, Vec<(f32, String)>>
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap();

    // Initial string: d
    let axiom = String::from("d");
    let mut stochastic_production_rules = HashMap::new();
    // Replacement rules:
    //    1)  d  d[cad]a   (p=50%)
    //    2)  d  d[bad]a   (p=50%)
    let mut choices = Vec::new();
    choices.push((0.5, String::from("d[cad]a")));
    choices.push((0.5, String::from("d[bad]a")));
    stochastic_production_rules.insert(String::from("d"), choices);



    Model { 
        axiom: axiom.clone(),
        path: axiom,
        iteration_count: 0,
        rules: stochastic_production_rules,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() % 30 == 0 {
        if model.iteration_count == 7 {
            // Reset the plant
            model.iteration_count = 0;
            model.path = produce_stochastic(&model.axiom, &model.rules);
        }
        else {
            // Keep growing
            model.iteration_count += 1;
            model.path = produce_stochastic(&model.path, &model.rules);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(WHITE);

    render_turtle(&draw, &model.path);
    draw.text(&app.fps().to_string()).x_y(-500.0, 500.0).color(FORESTGREEN);
    draw.to_frame(app, &frame).unwrap();

    if app.elapsed_frames() % 30 == 0 {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_project>/captures/<source_name>`.
        .join("captures")
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
