use std::fs::File;
use std::io::Read;

pub fn read_input() -> String {
    let file_path = "input.txt";
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
