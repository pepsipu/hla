use crate::ast;

#[derive(Debug)]
pub enum Registers {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP
}

#[derive(Debug)]
pub enum Data {
    Uint(u32),
    MemoryAddress(u32),
    Register(Registers),
    Expression(ast::Expression),
    Label(String),
    None,
}

#[derive(Debug)]
pub struct Condition {
    pub register: Registers,
    pub value: Data
}

#[derive(Debug)]
pub enum Jump {
    Je(String, Condition),
    Jne(String, Condition)
}

#[derive(Debug)]
pub enum Operator {
    Xor,
    And,
    Add,
}

#[derive(Debug)]
pub enum Assignee {
    Register(Registers),
    MemoryAddress(u32, Registers),
}

#[derive(Debug)]
pub struct Assignment {
    pub assignee: Assignee,
    pub value: Data,
}