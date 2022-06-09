use std::io::{self, Write};
use regex::Regex;


fn main() {
    print!("enter your weight on earth: ");
    io::stdout().flush().unwrap();

    let mut weight = String::new(); 
    io::stdin()
        .read_line(&mut weight)
        .expect("Falied to read_line");
    let weight = weight.trim();

    let weight = parse_weight(weight);

    println!("your weight on earth is {}kg", weight);
    println!("your weight on mars is {}kg", calc_mars_weight(weight));
}

fn parse_weight(weight: &str) -> f32 {
    let value_re: Regex = Regex::new(r"^(\d+\.?\d+)$").unwrap();
    let kg_re: Regex = Regex::new(r"^(\d+\.?\d+)kg$").unwrap();
    let g_re: Regex = Regex::new(r"^(\d+\.?\d+)g$").unwrap();

    if kg_re.is_match(weight) {
        parse_unit(kg_re, weight)
    } else if g_re.is_match(weight) {
        parse_unit(g_re, weight) / 1000.0
    } else {
        parse_unit(value_re, weight)
    }
}

fn parse_unit(unit_re: Regex, weight: &str) -> f32 {
    unit_re.captures(weight)
    .unwrap()
    .get(1)
    .unwrap()
    .as_str()
    .parse()
    .expect("Falied to parse weight")
}

fn calc_mars_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}