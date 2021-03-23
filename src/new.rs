use nannou::prelude::*;
use nannou::ui::prelude::*;

mod l_system;
pub use crate::l_system::*;
mod turtle;
pub use crate::turtle::Turtle;

pub fn render_turtle(draw: &Draw, path: &str, mut turtle: Turtle) {
    let scaling = 1.0;
    let turning_angle_increment = 5.0;
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
                turtle.turn(180.0);
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
                turtle.turn_x(turtle.turn_angle);
            } 
            // Roll counterclockwise
            "/" => {
                turtle.turn_y(turtle.turn_angle);
            }
            // Scale linelength
            "°" => {
                //turtle.line_length *= 0.9;
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

struct Model {
    ui: Ui,
    ids: Ids,
    turn_angle: f32,
    production: String,
    rotation: f32,
}

widget_ids! {
    struct Ids {
        turn_angle,
        rotation,
    }
}


fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap();

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();
    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    // 3d tree
    // let axiom = String::from("FFFA");
    // let mut production_rules = HashMap::new();
    // production_rules.insert(String::from("A"), String::from("°[^/FFFA]////[^/FFFA]////[^/FFFA]"));

    let l_system = fractal_plant();

    let mut production = l_system.axiom.clone();
    for _ in 0..6 {
        production = produce(&production, &l_system.production_rules)
    }

    Model {
        ui,
        ids,
        turn_angle: l_system.angle,
        production,
        rotation: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    let angle_slider = widget::Slider::new(model.turn_angle, 0.0, 90.0)
        .top_left_with_margin(20.0)
        .w_h(200.0, 30.0)
        .label("Turn Angle")
        .label_font_size(15)
        .label_rgb(1.0, 1.0, 1.0)
        .rgb(0.3, 0.3, 0.3)
        .border(0.0)    
        .set(model.ids.turn_angle, ui);

    let rotation_slider = widget::Slider::new(0.0, 0.0, 360.0)
        .align_left_of(model.ids.turn_angle)
        .w_h(200.0, 30.0)
        .label("Rotation")
        .label_font_size(15)
        .label_rgb(1.0, 1.0, 1.0)
        .rgb(0.3, 0.3, 0.3)
        .border(0.0)    
        .set(model.ids.rotation, ui);

    for value in angle_slider
    {
        model.turn_angle = value;
    }

    for value in rotation_slider
    {
        model.rotation = value;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(WHITE);

    let turtle = Turtle{
        position: vec3(
            app.window_rect().mid_bottom().x, 
            app.window_rect().bottom(),
            0.0),
        orientation: vec3(0.0, 1.0, 0.0),
        thickness: 2.0,
        color: FORESTGREEN,
        stack: Vec::new(),
        turn_angle: model.turn_angle,
        turn_reversed: false,
        line_length: 10.0,
    };

    render_turtle(&draw, &model.production, turtle);
    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}