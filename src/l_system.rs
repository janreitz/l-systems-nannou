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
