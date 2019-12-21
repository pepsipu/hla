use crate::ast;
use crate::structures;
use crate::syntax;

pub fn compile(root_ast: Vec<ast::Module>) ->(Vec<String>, Vec<String>) {
    let mut asm = Vec::<String>::new();
    let mut globals = Vec::<String>::new();
    for module in root_ast {
        match module {
            ast::Module::Raw(raw) => asm.push(raw),
            ast::Module::Label(label, global) => {
                if global {
                    globals.push(label.clone());
                }
                asm.push(format!("{}:", &label));
            }
            ast::Module::Statement(statement) => {
                match statement {
                    ast::Statement::Jump(jmp) => {
                        match jmp {
                            structures::Jump::Jne(label, condition) => {
                                asm.push(format!("cmp {}, {}", syntax::match_register_enum(condition.register), syntax::get_value(condition.value)));
                                asm.push(format!("jne {}", label))
                            },
                            structures::Jump::Je(label, condition) => {
                                asm.push(format!("cmp {}, {}", syntax::match_register_enum(condition.register), syntax::get_value(condition.value)));
                                asm.push(format!("je {}", label))
                            }
                        }
                    }
                    ast::Statement::Assignment(assignment) => {
                        match assignment.assignee {
                            structures::Assignee::Register(r) => {
                                let value = syntax::get_value(assignment.value);
                                let register = syntax::match_register_enum(r);
                                if &value == "0" {
                                    asm.push(format!("xor {}, {}", register, register));
                                } else {
                                    asm.push(format!("mov {}, {}", register, value))
                                }

                            },
                            structures::Assignee::MemoryAddress(address, register) => {
                                let register = syntax::match_register_enum(register);
                                asm.push(format!("mov {}, {}", register, syntax::get_value(assignment.value)));
                                asm.push(format!("mov [{}], {}", address.to_string(), register));
                            }
                        }
                    },
                    ast::Statement::Increment(register) => {
                        asm.push(format!("inc {}", syntax::match_register_enum(register)))
                    }
                    _ => {}
                }
            }
            _ => {}
        };
    }
    (asm, globals)
}