use crate::aoc_util;
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    register: String,
    instruction: i32,
    amount: i32,
    compare_register: String,
    compare_function: fn(i32, i32) -> bool,
    compare_amount: i32,
}

pub fn run() {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max_value = 0;

    for line in aoc_util::iterate_input_lines(8) {
        let res = parse_line(&line);
        if !registers.contains_key(&res.register) {
            registers.insert(res.register.clone(), 0);
        }

        if !registers.contains_key(&res.compare_register) {
            registers.insert(res.compare_register.clone(), 0);
        }

        let mut register_value = registers[&res.register];
        if (res.compare_function)(registers[&res.compare_register], res.compare_amount) {
            register_value += res.amount * res.instruction;
            registers.insert(res.register, register_value);
        }

        if register_value > max_value {
            max_value = register_value;
        }
    }
    println!("Part 1 is {}", registers.values().max().unwrap());
    println!("Part 2 is {}", max_value)
}

fn parse_line(line: &str) -> Line {
    let line_parts = line.split(' ').collect::<Vec<&str>>();

    let instruction = match line_parts[1] {
        "inc" => 1,
        "dec" => -1,
        _ => panic!("Bad instruction"),
    };

    let compare_function: fn(i32, i32) -> bool = match line_parts[5] {
        "<" => |a, b| a < b,
        "<=" => |a, b| a <= b,
        ">" => |a, b| a > b,
        ">=" => |a, b| a >= b,
        "==" => |a, b| a == b,
        "!=" => |a, b| a != b,
        _ => panic!("Instruction {} is not supported", line_parts[5]),
    };

    Line {
        register: String::from(line_parts[0]),
        instruction,
        amount: line_parts[2].parse().unwrap(),
        compare_register: String::from(line_parts[4]),
        compare_function,
        compare_amount: line_parts[6].parse().unwrap(),
    }
}
