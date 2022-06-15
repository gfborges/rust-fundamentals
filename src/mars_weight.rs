use regex::Regex;
use std::io::{self, Write};

fn main() {
    print!("enter your weight on earth: ");
    io::stdout().flush().unwrap();

    let mut weight = String::new();
    io::stdin()
        .read_line(&mut weight)
        .expect("Falied to read_line");
    let weight = weight.trim();

    let weight = parse_weight(weight).expect("Invalid input");

    println!("your weight on earth is {:.3}kg", weight);
    println!("your weight on mars is {:.3}kg", calc_mars_weight(weight));
}

fn parse_weight(weight: &str) -> Option<f32> {
    let value_re: Regex = Regex::new(r"^(\d+\.?\d+)$").unwrap();
    let kg_re: Regex = Regex::new(r"^(\d+\.?\d+)kg$").unwrap();
    let dg_re: Regex = Regex::new(r"^(\d+\.?\d+)dg$").unwrap();
    let g_re: Regex = Regex::new(r"^(\d+\.?\d+)g$").unwrap();

    if let Some(captures) = kg_re.captures(weight) {
        Option::Some(parse_unit(captures))
    } else if let Some(captures) = dg_re.captures(weight) {
        Option::Some(parse_unit(captures) / 100.0)
    } else if let Some(captures) = g_re.captures(weight) {
        Option::Some(parse_unit(captures) / 1000.0)
    } else if let Some(captures) = value_re.captures(weight) {
        Option::Some(parse_unit(captures))
    } else {
        Option::None
    }
}

fn parse_unit(captures: regex::Captures) -> f32 {
    captures
        .get(1)
        .expect("Falied t")
        .as_str()
        .parse()
        .expect("Falied to parse weight")
}

fn calc_mars_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
