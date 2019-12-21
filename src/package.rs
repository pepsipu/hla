use std::fs::File;
use std::io::Write;

pub fn create_file(asm: Vec<String>, globals: Vec<String>, name: String) {
    let mut contents = String::new();
    for label in globals {
        contents += &format!("global {}\n", label)
    }
    for statement in asm {
        contents += &format!("{}\n", statement)
    }
    File::create(name).unwrap().write_all(contents.as_bytes()).unwrap();
}