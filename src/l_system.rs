use std::collections::HashMap;
use rand::Rng;

pub fn produce(axiom: &str, rules: &HashMap<String, String>) -> String {
    let mut s = String::new();

    for var in axiom.chars() {
        match rules.get::<str>(&var.to_string()) {
            Some(string) => {
                // Variables are subsituted according to their production rules
                s.push_str(string)
            }
            None => {
                // Constants are simply kept 
                s.push_str(&var.to_string())
            }
        }
    };
    s
}

pub fn produce_stochastic(axiom: &str, rules: &HashMap<String, Vec<(f32, String)>>) -> String {
    let mut s = String::new();
    let mut rng = rand::thread_rng();

    for var in axiom.chars() {
        match rules.get::<str>(&var.to_string()) {
            Some(choices) => {
                // Variables are subsituted according to a production rule 
                // randomly chosen from the given choices
                let random_val: f32 = rng.gen(); 
                let mut total_prob = 0.0;
                for (prob, string) in choices {
                    total_prob += prob;
                    if total_prob > 1.0 {
                        // panic
                        println!("Total probability is greater than 1.0, aborting");
                        break;
                    }
                    if random_val < total_prob {
                        s.push_str(string);
                        break;
                    }
                }
            }
            None => {
                // Constants are simply kept 
                s.push_str(&var.to_string())
            }
        }
    };
    s
}

pub struct LSystem {
    pub axiom: String,
    pub production_rules: HashMap<String, String>,
    pub angle: f32,
}

pub fn fractal_plant() -> LSystem {
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("X"), String::from("F+[[X]-X]-F[-FX]+X"));
    production_rules.insert(String::from("F"), String::from("FF"));
    LSystem {
        axiom: String::from("X"),
        production_rules,
        angle: 25.0,
    }
}

pub fn fractal_tree() -> LSystem {
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("0"), String::from("1[0]0"));
    production_rules.insert(String::from("1"), String::from("11"));
    LSystem {
        axiom: String::from("0"),
        production_rules,
        angle: 25.0,
    }
}

pub fn square() -> LSystem {
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("X"), String::from("XF-F+F-XF+F+XF-F+F-X"));
    
    LSystem {
        axiom: String::from("F+XF+F+XF"),
        production_rules,
        angle: 90.0,
    }
}

pub fn hilbert() -> LSystem {
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("X"), String::from("-YF+XFX+FY-"));
    production_rules.insert(String::from("Y"), String::from("+XF-YFY-FX+"));
    
    LSystem {
        axiom: String::from("X"),
        production_rules,
        angle: 90.0,
    }
}

pub fn pentaplexity() -> LSystem {
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("F"), String::from("F++F++F|F-F++F"));
    
    LSystem {
        axiom: String::from("F++F++F++F++F"),
        production_rules,
        angle: 36.0,
    }
}

pub fn hexagonal_gosper() -> LSystem {
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("X"), String::from("X+YF++YF-FX--FXFX-YF+"));
    production_rules.insert(String::from("Y"), String::from("-FX+YFYF++YF+FX--FX-Y"));

    LSystem {
        axiom: String::from("XF"),
        production_rules,
        angle: 60.0,
    }
}

pub fn tree_3d() -> LSystem {
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("A"), String::from("[^FFFA]////[^FFFA]////[^FFFA]"));

    LSystem {
        axiom: String::from("FFFA"),
        production_rules,
        angle: 60.0,
    }
}

