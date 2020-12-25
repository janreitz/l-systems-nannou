use nannou::prelude::*;
use nannou::geom::Vector2;
use nannou::color::*;
use rand::Rng;

struct Leaf {
    position: f32,
    orientation: f32,
    offset: f32,
    size: f32,
    color: Hsv,
}

struct Node {
    children: Vec<Node>,
    leaves: Vec<Leaf>,
    thickness: f32,
    a: Vector2,
    b: Vector2,
}

impl Node {
    pub fn new(a: Vector2, b: Vector2) -> Self {
        Node {
            children: Vec::new(),
            leaves: Vec::new(),
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
        let angle =  mean_regression * current_angle + (1.0 - mean_regression) * deg_to_rad(random_range::<f32>(-10.0, 10.0)); 
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

        // Grow leaves
        let mut rng = rand::thread_rng();
        // The thinner the branch the more leaves it has
        let num_leaves = (rng.gen::<f32>() * 15.0 / self.thickness) as i32;
        let leave_diff = num_leaves - self.leaves.len() as i32; 

        if leave_diff < 0 {
            // remove leaves
            for _ in 0..-leave_diff {
                self.leaves.pop();
            }
        }
        else if leave_diff > 0 {
            // add leaves
            let col1 = hsv(0.0,0.0,1.0);
            let col2 = hsv(1.0,1.0,1.0);
    
            for _ in 0..leave_diff {
                self.leaves.push(Leaf{
                    orientation: rng.gen::<f32>() * 2.0 * PI,
                    position: rng.gen::<f32>(),
                    offset: rng.gen::<f32>() * 50.0,
                    size: rng.gen::<f32>() * 15.0,
                    color: col1.mix(&col2, rng.gen::<f32>()), 
                })
            }
        }
        self.update_thickness();
    }
    
    // Takes the nannou::Draw API 
    pub fn draw(&self, draw: &Draw) {
        // draw the branch
        draw.line()
            .start(self.a)
            .end(self.b)
            .caps_round()
            .weight(self.thickness)
            .color(BROWN);
        
        let shape = self.b - self.a;
        // Draw leaves around the branch
        for leaf in self.leaves.iter() {
            let start = self.a + shape * leaf.position + shape.normalize().rotate(leaf.orientation) * leaf.offset;
            
            draw.line()
                .start(start)
                .end(start + shape.normalize().rotate(leaf.orientation) * leaf.size)
                .weight(leaf.size * 0.66)
                .caps_round()
                .color(leaf.color);
        }
        for child in self.children.iter() {
            child.draw(draw)
        }
    }
}

struct Model {
    node: Node,
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_project>/captures/<source_name>`.
        .join("captures")
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("img{:04}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap(); 

    Model { 
        node: Node::new(
            vec2(0.0, -512.0),
            vec2(0.0, -502.0),
        )
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
        let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn main() {
    nannou::app(model).update(update).run();
}