use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn create_file(asm: Vec<String>, globals: Vec<String>, name: String) {
    let mut contents = String::new();
    for label in globals {
        contents += &format!("global {}\n", label)
    }
    for statement in asm {
        contents += &format!("{}\n", statement)
    }
    File::create(Path::new(&name).file_stem().unwrap().to_str().unwrap().to_owned() + ".S").unwrap().write_all(contents.as_bytes()).unwrap();
}