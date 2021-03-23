mod l_system;
mod turtle;
mod capture;
use crate::l_system::*;
use crate::turtle::Turtle;
use crate::capture::{
    capture_path_timestamp
};

use nannou::{
    math::Deg, 
    prelude::*, 
    ui::{
        prelude::*,
        widget::Slider
    }};

fn main() {
    nannou::app(model).update(update).run();
}


fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            match key {
                Key::H => { model.hide_ui = !model.hide_ui; }
                Key::C => { model.capture_image = true; }
                _ => {}
            }
        }
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

struct Model {
    ui: Ui,
    ids: Ids,
    turn_angle: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,
    scale: f32,
    l_system: LSystem,
    iterations: i32,
    production: String,
    capture_image: bool,
    hide_ui: bool,
}

widget_ids! {
    struct Ids {
        turn_angle,
        roll,
        pitch,
        yaw,
        scale,
        capture_image,
        iterations,
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
    .size(1024,1024)
    .view(view)
    .event(window_event)
    .build()
    .unwrap();

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();
    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    let l_system = tree_3d();

    let mut production = l_system.axiom.clone();
    for _ in 0..3 {
        production = produce(&production, &l_system.production_rules)
    }

    Model {
        ui,
        ids,
        turn_angle: l_system.angle,
        iterations: 4,
        roll: 0.0,
        pitch: 0.0,
        yaw: 0.0,
        scale: 1.0,
        production,
        l_system,
        capture_image: false,
        hide_ui: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    fn slider(value: f32, min: f32, max: f32) -> Slider<'static, f32> {
        widget::Slider::new(value, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .label_rgb(1.0, 1.0, 1.0)
            .rgb(0.3, 0.3, 0.3)
            .border(0.0)   
    }

    let angle_slider = slider(model.turn_angle, 0.0, 90.0)
        .top_left_with_margin(20.0)
        .label("Branch Angle")
        .set(model.ids.turn_angle, ui);

    let roll_slider = slider(0.0, 0.0, 360.0)
        .down(10.0)
        .label("Roll")
        .set(model.ids.roll, ui);
    
    let pitch_slider = slider(0.0, 0.0, 360.0)
        .down(10.0)
        .label("Pitch")
        .set(model.ids.pitch, ui);
    
    let yaw_slider = slider(0.0, 0.0, 360.0)
        .down(10.0)
        .label("Yaw")
        .set(model.ids.yaw, ui);
    
    let scale_slider = slider(model.scale, 0.0, 5.0)
        .down(10.0)
        .label("Scale")
        .set(model.ids.scale, ui);

    let iterations_slider = widget::Slider::new(model.iterations as f32, 0.0, 10.0)
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .label_rgb(1.0, 1.0, 1.0)
        .rgb(0.3, 0.3, 0.3)
        .border(0.0)    
        .down(10.0)
        .label("Iterations")
        .set(model.ids.iterations, ui);

    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 30.0)
        .label("Capture [C]")
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.capture_image, ui) {
        model.capture_image = true;
    }
        
    for value in angle_slider { model.turn_angle = value; }
    for value in roll_slider { model.roll = value; }
    for value in pitch_slider { model.pitch = value; }
    for value in yaw_slider { model.yaw = value; }
    for value in scale_slider { model.scale = value; }
    for value in iterations_slider { 
        model.iterations = value as i32; 
        let mut production = model.l_system.axiom.clone();
        for _ in 0..value as i32 {
            production = produce(&production, &model.l_system.production_rules)
        }
        model.production = production;
    }

    if model.capture_image {
        let file_path = capture_path_timestamp(app);
        app.main_window().capture_frame(file_path);
        model.capture_image = false;
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(WHITE);

    let mut turtle = Turtle{
        position: vec3(
            0.0,//app.window_rect().mid_bottom().x, 
            0.0, //app.window_rect().bottom(),
            0.0).into(),
        thickness: 2.0,
        color: FORESTGREEN,
        stack: Vec::new(),
        turn_angle: Deg(model.turn_angle),
        turn_reversed: false,
        line_length: 10.0,
        .. Turtle::default()
    };

    turtle.roll(Deg(model.roll));
    turtle.pitch(Deg(model.pitch));
    turtle.yaw(Deg(model.yaw));

    render_turtle(&draw, &model.production, turtle, model.scale);
    draw.to_frame(app, &frame).unwrap();
    
    if model.capture_image {
        
    }
    
    if !model.hide_ui {
        model.ui.draw_to_frame(app, &frame).unwrap();
    }
}

pub fn render_turtle(draw: &Draw, path: &str, mut turtle: Turtle, scaling: f32) {
    let turning_angle_increment = Deg(5.0);
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
                turtle.yaw(-turtle.turn_angle);
            }
            // Turn right by turning angle
            "-" => {
                turtle.yaw(turtle.turn_angle);
            }
            // Reverse direction (ie: turn by 180 degrees)
            "|" => {
                turtle.yaw(Deg(180.0));
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
            // Pitch upwards
            "^" => {
                turtle.pitch(turtle.turn_angle);
            } 
            // Roll counterclockwise
            "/" => {
                turtle.roll(turtle.turn_angle);
            }
            // Scale linelength
            "°" => {
                turtle.line_length *= 0.9;
            }
            // X does not correspond to any drawing action and is used to control the evolution of the curve. 
            "X" => {}
            // Y does not correspond to any drawing action and is used to control the evolution of the curve. 
            "Y" => {}
            // A does not correspond to any drawing action and is used to control the evolution of the curve. 
            "A" => {}
            
            _ => {
                println!("unknown command")
            }
        }
    }
}
