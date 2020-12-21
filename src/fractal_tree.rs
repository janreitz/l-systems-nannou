use std::collections::HashMap;

mod l_system;
pub use crate::l_system::produce;

fn main() {
    let axiom = String::from("0");
    let mut production_rules = HashMap::new();
    production_rules.insert(String::from("0"), String::from("1[0]0"));
    production_rules.insert(String::from("1"), String::from("11"));

    let mut production = axiom;
    for _i in 0..4 {
        production = produce(&production, &production_rules);
        println!("{}", production);
    }
}