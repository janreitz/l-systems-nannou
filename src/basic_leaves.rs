use nannou::prelude::*;
use nannou::geom::Vector2;
use nannou::color::*;
use rand::Rng;

fn to_rad(deg: f32) -> f32 {
    PI/180.0*deg
}  

struct Node {
    children: Vec<Node>,
    thickness: f32,
    a: Vector2,
    b: Vector2,
}

impl Node {
    pub fn new(a: Vector2, b: Vector2) -> Self {
        Node {
            children: Vec::new(),
            thickness: 1.0,
            a,
            b,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn update_thickness(& mut self) {
        let mut sum_squared_thicknesses = 1.0;
        for child in self.children.iter() {
            sum_squared_thicknesses += child.thickness.powi(2);
        }
        self.thickness = sum_squared_thicknesses.sqrt()
    }

    pub fn branch(& mut self) {
        let my_shape = self.b - self.a;
        let mut current_angle = 0.0;
        if my_shape.x == 0.0 {
            current_angle = (my_shape.x/my_shape.y).tan() 
        }
        let mean_regression = 0.02;
        let angle =  mean_regression * current_angle + (1.0 - mean_regression) * to_rad(random_range::<f32>(-10.0, 10.0)); 
        let new_shape = my_shape.rotate(angle); 
        let branch = Node::new(self.b, self.b + new_shape);
        self.children.push(branch)
    }

    pub fn update(& mut self) {
        for child in self.children.iter_mut() {
            child.update();
        }
        // Leaves always grow
        if self.is_leaf() {
            self.branch();
        }
        // Random branching, dependent on thickness
        else if random_range::<f32>(0.0, 50.0) < 1.0/self.thickness.powi(2) {
            self.branch();
        }
        self.update_thickness();
    }

    // Takes the nannou::Draw API 
    pub fn draw(&self, draw: &Draw) {
        draw.line()
            .start(self.a)
            .end(self.b)
            .caps_round()
            .weight(self.thickness)
            .color(BROWN);
        
        let shape = self.b - self.a;
        // Draw leaves around the branch
        let mut rng = rand::thread_rng();
        let col1 = hsv(0.0,0.0,1.0);
        let col2 = hsv(1.0,1.0,1.0);
        // The thinner the branch the more leaved it has
        let num_leaves = (rng.gen::<f32>() * 10.0 / self.thickness).floor() as u8;

        for _ in 0..num_leaves {
            let orientation = rng.gen::<f32>() * 2.0 * PI;
            let position = self.a + shape * rng.gen::<f32>();
            let size = rng.gen::<f32>() * 15.0;
            let color = col1.mix(&col2, rng.gen::<f32>()); 

            draw.line()
                .start(position)
                .end(position + shape.normalize().rotate(orientation) * size)
                .weight(size * 0.66)
                .caps_round()
                .color(color);
        }
        for child in self.children.iter() {
            child.draw(draw)
        }
    }
}

struct Model {
    node: Node,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap();
    let node = Node::new(
        vec2(0.0, -512.0),
        vec2(0.0, -502.0),
        // vec2(1024.0/2.0, 1024.0),
        // vec2(1024.0/2.0, 1014.0),
    );

    Model { 
        node: node,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() % 10 == 0 {
        model.node.update()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    model.node.draw(&draw);
    draw.text(&app.fps().to_string())
        .x_y(0.0, 0.0)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}