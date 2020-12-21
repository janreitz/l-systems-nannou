use std::collections::HashMap;

pub fn produce(axiom: &str, rules: &HashMap<&str, &str>) -> String {
    let mut s = String::new();

    for var in axiom.chars() {
        match rules.get::<str>(&var.to_string()) {
            Some(string) => {
                s.push_str(string)
            }
            None => println!("This appears to be a constant"),
        }
    };
    s
}

fn main() {
    let axiom = String::from("AB");
    let mut production_rules = HashMap::new();
    production_rules.insert("A", "AB");
    production_rules.insert("B", "A");

    let mut production = axiom;
    for _i in 0..10 {
        production = produce(&production, &production_rules);
        println!("{}", production);
    }
}