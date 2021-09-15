use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Registers = BTreeMap<String, i32>;
type InstructionList = Vec<Instruction>;

struct Process {
    registers: Registers,
}

struct Instruction {
    condition: Condition,
    operation: Operation,
}

struct Condition {
    reg_name: String,
    condition: CmpOp,
}

enum CmpOp {
    Eq(i32),
    Neq(i32),
    Gt(i32),
    Gte(i32),
    Lt(i32),
    Lte(i32),
}

struct Operation {
    reg_name: String,
    operation: ArithOp,
}

enum ArithOp {
    Inc(i32),
    Dec(i32),
}

fn main() {
    let (registers, instructions) = parse_input();
    let mut process = Process {registers};

    let mut max_during = 0;

    for instruction in &instructions {
        if process.evaluate(&instruction.condition) {
            process.execute(&instruction.operation);

            let reg_name: &str = &instruction.operation.reg_name;
            let reg_val = *process.registers.get(reg_name).unwrap();
            if reg_val > max_during { max_during = reg_val; }
        }
    }

    let mut max_after = i32::MIN;
    for &v in process.registers.values() {
        if v > max_after { max_after = v; }
    }

    println!("Part1: max after finish={}", max_after);
    println!("Part2: max while running={}", max_during);
}

impl Process {
    fn evaluate(&self, condition: &Condition) -> bool {
        let reg_val = *self.registers.get(&condition.reg_name).unwrap();
        match condition.condition {
            CmpOp::Eq(v)  => reg_val == v,
            CmpOp::Neq(v) => reg_val != v,
            CmpOp::Gt(v)  => reg_val > v,
            CmpOp::Gte(v) => reg_val >= v,
            CmpOp::Lt(v)  => reg_val < v,
            CmpOp::Lte(v) => reg_val <= v,
        }
    }

    fn execute(&mut self, operation: &Operation) {
        let reg_val = self.registers.get_mut(&operation.reg_name).unwrap();
        match operation.operation {
            ArithOp::Inc(v) => *reg_val += v,
            ArithOp::Dec(v) => *reg_val -= v,
        };
    }
}

fn parse_input() -> (Registers, InstructionList) {
    let f = File::open("input.txt").unwrap();
    let rdr = BufReader::new(f);

    let mut registers = Registers::new();
    let mut instructions = InstructionList::new();

    for line in rdr.lines().map(|l| l.unwrap()) {
        let instruction = parse_line(&line);
        registers.insert(String::from(&instruction.condition.reg_name), 0);
        registers.insert(String::from(&instruction.operation.reg_name), 0);
        instructions.push(instruction);
    }

    (registers, instructions)
}

fn parse_line(line: &str) -> Instruction {
    let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[3], "if");

    let reg = tokens[4];
    let val = tokens[6].parse::<i32>().unwrap();
    let op  = match tokens[5] {
        "==" => CmpOp::Eq(val),
        "!=" => CmpOp::Neq(val),
        ">"  => CmpOp::Gt(val),
        ">=" => CmpOp::Gte(val),
        "<"  => CmpOp::Lt(val),
        "<=" => CmpOp::Lte(val),
        _ => panic!(),
    };
    let condition = Condition {
        reg_name: String::from(reg),
        condition: op,
    };

    let reg = tokens[0];
    let val = tokens[2].parse::<i32>().unwrap();
    let op  = match tokens[1] {
        "inc" => ArithOp::Inc(val),
        "dec" => ArithOp::Dec(val),
        _ => panic!(),
    };
    let operation = Operation {
        reg_name: String::from(reg),
        operation: op,
    };

    Instruction {condition, operation}
}