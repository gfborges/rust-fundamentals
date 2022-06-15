use std::fs::File;
use std::io::{self, Read};

fn main() {
    match read_file("~/hello.txt") {
        Ok(content) => println!("{}", content),
        Err(_) => println!("Bad file name =("),
    }
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
