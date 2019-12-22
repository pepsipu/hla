#[macro_use]
extern crate nom;

#[macro_use]
extern crate lazy_static;

mod ast;
mod syntax;
mod compiler;
mod package;
mod structures;

use std::process::exit;
use std::sync::Mutex;
use std::env::args;

lazy_static! {
    static ref LABELS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

fn main() -> Result<(), std::io::Error> {
    let file_contents = std::fs::read_to_string(args().nth(1).unwrap())?;
    let program = file_contents.split("\n").collect::<Vec<&str>>();
    let mut root_ast = Vec::<ast::Module>::new();
    for untrimmed_line in program {
        let line = untrimmed_line.trim();
        if line != "" && !line.starts_with(";") {
            let module = syntax::parse(line);
            match module {
                Some(i) => root_ast.push(i),
                None => {}
            }
        }

    }
    println!("{:?}", root_ast);
    let (asm, globals) = compiler::compile(root_ast);
    println!("{:?}", asm);
    package::create_file(asm, globals, args().nth(1).unwrap());
    Ok(())
}
