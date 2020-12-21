use std::collections::HashMap;

pub fn produce(axiom: &str, rules: &HashMap<&str, &str>) -> String {
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
