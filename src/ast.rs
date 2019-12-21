use crate::structures;

#[derive(Debug)]
pub enum Expression {

}

#[derive(Debug)]
pub enum Module {
    Label(String, bool), // label name, if label is a symbol
    Statement(Statement),
    Raw(String),
    None
}

#[derive(Debug)]
pub enum Statement {
    Assignment(structures::Assignment),
    Jump(structures::Jump),
    Increment(structures::Registers),
    None
}