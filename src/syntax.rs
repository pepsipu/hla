use std::process::exit;
use crate::ast;
use crate::structures::*;
use crate::structures::Data::Register;

named!(assign<&str, (&str, &str, &str)>,
    do_parse!(
        token: take!(1) >>
        register: take_until!("=") >>
        tag!("=") >>
        value: take_until!("\0") >>
        (token, register.trim(), value.trim())
    )
);

fn is_operation(x: char) -> bool {
    x == '=' || x == '!' || x == '\0'
}

named!(cond<&str, (&str, &str, &str)>,
    do_parse!(
        arg: take_till!(is_operation) >>
        op: take_while!(is_operation) >>
        arg2: take_till!(is_operation) >>
        (op, arg.trim(), arg2.trim())
    )
);

fn is_not_label(x: char) -> bool {
    x == '(' || x == '\0'
}

named!(read_label<&str, &str>,
    do_parse!(
        label: take_till!(is_not_label) >>
        (label.trim())
    )
);

named!(jmp<&str, (&str, &str)>,
    do_parse!(
        ws!(take_until!(" ")) >>
        label: take_until!(" ") >>
        take_until!("(") >>
        take!(1) >>
        condition: take_until!(")") >>
        (label, condition.trim())
    )
);

named!(mem_write<&str, (&str, &str)>,
    do_parse!(
        address: take_until!("(") >>
        take!(1) >>
        register: take_until!(")") >>
        (address.trim(), register.trim())
    )
);

named!(resv<&str, u32>,
    do_parse!(
        take_until!("[") >>
        take!(1) >>
        value: take_until!("]") >>
        (value.parse::<u32>().unwrap())
    )
);

pub fn match_register_str(register: &str) -> Registers {
    match register {
        "eax" => Registers::EAX,
        "ebx" => Registers::EBX,
        "ecx" => Registers::ECX,
        "edx" => Registers::EDX,
        "esi" => Registers::ESI,
        "edi" => Registers::EDI,
        "esp" => Registers::ESP,
        "ebp" => Registers::EBP,
        _ => {
            println!("register {} be like: gone", register);
            exit(2)
        }
    }
}

pub fn get_value(data: Data) -> String {
    match data {
        Data::MemoryAddress(address) => {
            format!("[{}]", address.to_string())
        }
        Data::Uint(uint) => {
            uint.to_string()
        }
        Data::Label(label) => {
            label
        },
        Data::Register(register) => {
            String::from(match_register_enum(register))
        }
        _ => {
            println!("data {:?} be like: not parsable", data);
            exit(2)
        }
    }
}

pub fn match_register_enum(register: Registers) -> &'static str {
    match register {
        Registers::EAX => "eax",
        Registers::EBX => "ebx",
        Registers::ECX => "ecx",
        Registers::EDX => "edx",
        Registers::ESI => "esi",
        Registers::EDI => "edi",
        Registers::ESP => "esp",
        Registers::EBP => "ebp",
        Registers::None => {
            println!("register cannot be none");
            exit(2)
        }
    }
}

pub fn parse_numerical(string_value: &str) -> Option<u32> {
    if string_value.starts_with("0x") {
        Some(u32::from_str_radix(&string_value[2..], 16).unwrap())
    } else if string_value.starts_with("0b") {
        Some(u32::from_str_radix(&string_value[2..], 2).unwrap())
    } else if string_value.parse::<u32>().is_ok() {
        Some(string_value.parse::<u32>().unwrap())
    } else {
        None
    }
}

pub fn parse_value(string_value: &str) -> Data {
    let labels = crate::LABELS.lock().unwrap();
    let str_value = String::from(string_value);
    if labels.contains(&str_value) {
        Data::Label(str_value)
    } else {
        return match parse_numerical(&string_value) {
            Some(i) => {
                return Data::Uint(i);
            }
            None => {
                if string_value.starts_with("$") {
                    Data::Register(match_register_str(&string_value[1..]))
                } else if string_value.starts_with("*") {
                    Data::MemoryAddress(parse_numerical(&string_value[1..]).unwrap())
                } else {
                    Data::None
                }
            }
        };
    }
}

pub fn parse_assign(assignment: &str) -> Assignment {
    let formatted_assignment = format!("{}\0", &assignment.trim());
    let (string_token, string_assignment, string_value) = assign(&formatted_assignment).unwrap().1;
    let assignee: Assignee = match string_token {
        "$" => Assignee::Register(match_register_str(string_assignment)),
        "*" => {
            let labels = crate::LABELS.lock().unwrap();
            let str_value = String::from(string_assignment);
            let formatted_read_label = format!("{}\0", str_value);
            let parsed_label = read_label(&formatted_read_label);
            if labels.contains(&String::from(parsed_label.unwrap().1)) {
                if str_value.contains("$") {
                    let (label, register) = mem_write(&string_assignment).unwrap().1;
                    Assignee::Label(String::from(label), match_register_str(&register[1..]))
                } else {
                    Assignee::Label(str_value, Registers::None)
                }
            } else {
                if string_assignment.contains("$") {
                    let (address, register) = mem_write(&string_assignment).unwrap().1;
                    Assignee::MemoryAddress(parse_numerical(&address).unwrap(), match_register_str(register))

                } else {
                    Assignee::MemoryAddress(parse_numerical(&string_assignment).unwrap(), Registers::None)
                }
            }
        }
        _ => exit(4)
    };
    Assignment {
        assignee,
        value: parse_value(string_value),
    }
}

pub fn parse(statement: &str) -> Option<ast::Module> {
    if statement.starts_with("$") || statement.starts_with("*") {
        if statement.ends_with("++") {
            Some(ast::Module::Statement(ast::Statement::Increment(match_register_str(&statement[1..statement.len() - 2]))))
        } else {
            Some(ast::Module::Statement(ast::Statement::Assignment(parse_assign(statement))))
        }
    } else if statement.ends_with(":") {
        if statement.starts_with("@") {
            let label = String::from(&statement[1..statement.len() - 1]);
            crate::LABELS.lock().unwrap().push(label.clone());
            Some(ast::Module::Label(label, true))
        } else {
            let label = String::from(&statement[..statement.len() - 1]);
            crate::LABELS.lock().unwrap().push(label.clone());
            Some(ast::Module::Label(label, false))
        }
    } else if statement.starts_with("goto ") {
        if statement.contains("if") {
            let (label, condition) = jmp(statement).unwrap().1;
            let expression = format!("{}\0", &condition);
            let (comparison, register, value) = cond(&expression).unwrap().1;
            Some(ast::Module::Statement(ast::Statement::Jump(match comparison {
                "!=" => Jump::Jne(String::from(label), Condition {
                    register: match_register_str(&register[1..]),
                    value: parse_value(value),
                }),
                "==" => Jump::Je(String::from(label), Condition {
                    register: match_register_str(&register[1..]),
                    value: parse_value(value),
                }),
                _ => {
                    println!("invalid jump condition");
                    exit(2);
                }
            })))
        } else {
            let label = statement.split(" ").collect::<Vec<&str>>()[1];
            Some(ast::Module::Statement(ast::Statement::Jump(Jump::Jmp(String::from(label)))))
        }

    } else if statement.starts_with("!") {
        Some(ast::Module::Raw(String::from(&statement[1..])))
    } else if statement.starts_with("const ") {
        Some(ast::Module::Statement(ast::Statement::Constant(String::from(&statement[6..]))))
    } else if statement.starts_with("reserve") {
        Some(ast::Module::Statement(ast::Statement::Reserve(resv(statement).unwrap().1)))
    } else {
        println!("could not parse: {}", statement);
        None
    }
}

pub fn parse_expression(expr: &str) {}
