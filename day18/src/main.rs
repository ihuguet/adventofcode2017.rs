use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, VecDeque};
use std::fmt::{self, Display};

fn main() {
    let instructions = parse_instructions("input.txt");

    let snd_recovered = run_v1(instructions.clone()).unwrap();
    println!("Part 1: recovered_snd={}", snd_recovered);

    let count_send_id1 = run_v2(instructions).unwrap();
    println!("Part 2: count_snd_ID1={}", count_send_id1);
}

type Registers = HashMap<char,i64>;
type Instructions = Vec<Instruction>;

#[derive(Debug,Clone)]
enum Instruction {
    Snd(Operand),
    Set(Operand,Operand),
    Add(Operand,Operand),
    Mul(Operand,Operand),
    Mod(Operand,Operand),
    Rcv(Operand),
    Jgz(Operand,Operand),
}

#[derive(Debug,Clone,Copy)]
enum Operand {
    Register(char),
    Number(i64),
}

trait OperandReader {
    fn val(&self, operand: Operand) -> i64;
    fn reg_mut(&mut self, operand: Operand) -> &mut i64;
}

impl OperandReader for Registers {
    fn val(&self, operand: Operand) -> i64 {
        match operand {
            Operand::Register(r) => *self.get(&r).unwrap_or(&0),
            Operand::Number(n) => n,
        }
    }

    fn reg_mut(&mut self, operand: Operand) -> &mut i64 {
        let reg = match operand {
            Operand::Register(ch) => ch,
            _ => panic!(),
        };
        self.entry(reg).or_default()
    }
}

fn run_v1(prog: Instructions) -> Option<i64> {
    let mut regs = Registers::new();
    let mut pc = 0usize;
    let mut last_snd = 0i64;

    loop {    
        let mut pc_inc: isize = 1;

        match prog[pc] {
            Instruction::Snd(op) => last_snd = regs.val(op),
            Instruction::Set(dst,src) => *regs.reg_mut(dst) = regs.val(src),
            Instruction::Add(dst,src) => *regs.reg_mut(dst) += regs.val(src),
            Instruction::Mul(dst,src) => *regs.reg_mut(dst) *= regs.val(src),
            Instruction::Mod(dst,src) => *regs.reg_mut(dst) %= regs.val(src),
            Instruction::Rcv(op) => if regs.val(op) != 0 {
                    return Some(last_snd);
                },
            Instruction::Jgz(cond,jump) => if regs.val(cond) > 0 {
                    pc_inc = regs.val(jump) as isize;
                },
        }

        let next_pc = pc as isize + pc_inc;
        if next_pc >= 0 && next_pc < prog.len() as isize {
            pc = next_pc as usize;
        } else {
            return None;
        }
    }
}

#[derive(Default)]
struct ThreadV2 {
    regs: Registers,
    queue: VecDeque<i64>,
    pc: usize,
    msgs_sent: u64,
    waiting: bool,
    finished: bool,
}

fn run_v2(prog: Instructions) -> Option<u64> {
    let mut thread0: ThreadV2 = Default::default();
    let mut thread1: ThreadV2 = Default::default();

    thread0.regs.insert('p', 0);
    thread1.regs.insert('p', 1);

    let mut active = &mut thread0;
    let mut sleeping = &mut thread1;

    loop {
        let mut pc_inc: isize = 1;

        match prog[active.pc] {
            Instruction::Snd(op) => {
                    sleeping.queue.push_back(active.regs.val(op));
                    sleeping.waiting = false;
                    active.msgs_sent += 1;
                },
            Instruction::Set(dst,src) => *active.regs.reg_mut(dst) = active.regs.val(src),
            Instruction::Add(dst,src) => *active.regs.reg_mut(dst) += active.regs.val(src),
            Instruction::Mul(dst,src) => *active.regs.reg_mut(dst) *= active.regs.val(src),
            Instruction::Mod(dst,src) => *active.regs.reg_mut(dst) %= active.regs.val(src),
            Instruction::Rcv(dst) => {
                    if let Some(val) = active.queue.pop_front() {
                        *active.regs.reg_mut(dst) = val;
                    }
                    else {
                        active.waiting = true;
                        pc_inc = 0;
                    }
                },
            Instruction::Jgz(cond,jump) => if active.regs.val(cond) > 0 {
                    pc_inc = active.regs.val(jump) as isize;
                },
        }

        let next_pc = active.pc as isize + pc_inc;
        if next_pc >= 0 && next_pc < prog.len() as isize {
            active.pc = next_pc as usize;
        } else {
            active.finished = true;
        }

        if active.waiting {
            if sleeping.waiting || sleeping.finished {
                active.finished = true;
                sleeping.finished = true;
                break;
            }
            let tmp = active;
            active = sleeping;
            sleeping = tmp;
        }
    }

    Some(thread1.msgs_sent)
}

fn parse_instructions(filename: &str) -> Instructions {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split_whitespace();

        let operation = split.next().unwrap();
        let instruction = match operation {
            "snd" => {
                let op1 = parse_operand(split.next().unwrap());
                Instruction::Snd(op1)
            },
            "set" => {
                let op1 = parse_operand(split.next().unwrap());
                let op2 = parse_operand(split.next().unwrap());
                assert_is_register(op1);
                Instruction::Set(op1, op2)
            },
            "add" => {
                let op1 = parse_operand(split.next().unwrap());
                let op2 = parse_operand(split.next().unwrap());
                assert_is_register(op1);
                Instruction::Add(op1, op2)
            },
            "mul" => {
                let op1 = parse_operand(split.next().unwrap());
                let op2 = parse_operand(split.next().unwrap());
                assert_is_register(op1);
                Instruction::Mul(op1, op2)
            },
            "mod" => {
                let op1 = parse_operand(split.next().unwrap());
                let op2 = parse_operand(split.next().unwrap());
                assert_is_register(op1);
                Instruction::Mod(op1, op2)
            },
            "rcv" => {
                let op1 = parse_operand(split.next().unwrap());
                Instruction::Rcv(op1)
            },
            "jgz" => {
                let op1 = parse_operand(split.next().unwrap());
                let op2 = parse_operand(split.next().unwrap());
                Instruction::Jgz(op1, op2)
            },
            _ => panic!(),
        };

        instructions.push(instruction);
    }

    instructions
}

fn parse_operand(op_str: &str) -> Operand {
    if let Ok(num) = op_str.parse() {
        Operand::Number(num)
    } else if op_str.len() == 1 {
        Operand::Register(op_str.chars().next().unwrap())
    } else {
        panic!()
    }
}

fn assert_is_register(op: Operand) {
    if let Operand::Number(_) = op {
        panic!();
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Register(r) => write!(f, "{}", r),
            Operand::Number(n) => write!(f, "{}", n),
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_v1() {
        let instructions = parse_instructions("input_test.txt");
        let snd_recovered = run_v1(instructions).unwrap();
        assert_eq!(snd_recovered, 4);
    }

    #[test]
    fn program_v2() {
        let instructions = parse_instructions("input_test2.txt");
        let count_send1 = run_v2(instructions).unwrap();
        assert_eq!(count_send1, 3);
    }
}