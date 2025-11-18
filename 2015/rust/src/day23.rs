use crate::aoc::Solution;
use crate::day23::Op::{Hlf, Inc, Jie, Jio, Jmp, Tpl};
use std::fmt::{Debug, Display, Formatter};

pub fn solve(input: &str) -> Solution {
    let instructions = parse(input);

    let mut regs = run(RegisterSet(0, 0), &instructions);
    let part1 = regs.get(&Register::B);

    let mut regs = run(RegisterSet(1, 0), &instructions);
    let part2 = regs.get(&Register::B);
    Solution::new(23, &part1, &part2)
}

#[derive(Debug)]
enum Register {
    A,
    B,
}
impl Register {
    fn from(register: &str) -> Option<Register> {
        match register {
            "a" => Some(Register::A),
            "b" => Some(Register::B),
            _ => None,
        }
    }
}
impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::A => write!(f, "a")?,
            Register::B => write!(f, "b")?,
        }
        Ok(())
    }
}

struct RegisterSet(u32, u32);
impl RegisterSet {
    fn get(&mut self, register: &Register) -> &mut u32 {
        match register {
            Register::A => &mut self.0,
            Register::B => &mut self.1,
        }
    }
}

#[derive(Debug)]
enum Op {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}
impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn write_offset(f: &mut Formatter<'_>, n: i32) -> std::fmt::Result {
            if n > 0 {
                write!(f, "+{n}")?;
            } else {
                write!(f, "{n}")?;
            }
            Ok(())
        }
        match self {
            Hlf(reg) => write!(f, "hlf {reg}")?,
            Tpl(reg) => write!(f, "tpl {reg}")?,
            Inc(reg) => write!(f, "inc {reg}")?,
            Jmp(offset) => {
                write!(f, "jmp ")?;
                write_offset(f, *offset)?;
            }
            Jie(reg, offset) => {
                write!(f, "jie {reg} ")?;
                write_offset(f, *offset)?;
            }
            Jio(reg, offset) => {
                write!(f, "jio {reg} ")?;
                write_offset(f, *offset)?;
            }
        }
        Ok(())
    }
}

fn parse(input: &str) -> Vec<Op> {
    let mut ops = Vec::new();
    for line in input.lines() {
        let mut parts = line.splitn(2, ' ');
        if let (Some(instruction), Some(args)) = (parts.next(), parts.next()) {
            let op = match instruction {
                "hlf" => Hlf(Register::from(args).unwrap()),
                "tpl" => Tpl(Register::from(args).unwrap()),
                "inc" => Inc(Register::from(args).unwrap()),
                "jmp" => Jmp(args.parse().unwrap()),
                "jie" => {
                    let mut args_pargs = args.split(", ");
                    let register = Register::from(args_pargs.next().unwrap()).unwrap();
                    let offset = args_pargs.next().unwrap().parse().unwrap();
                    Jie(register, offset)
                }
                "jio" => {
                    let mut args_pargs = args.split(", ");
                    let register = Register::from(args_pargs.next().unwrap()).unwrap();
                    let offset = args_pargs.next().unwrap().parse().unwrap();
                    Jio(register, offset)
                }
                _ => unimplemented!(),
            };
            ops.push(op);
        }
    }

    ops
}

fn run(regs: RegisterSet, instructions: &[Op]) -> RegisterSet {
    let mut regs = regs;
    let mut ip: i32 = 0;

    while ip >= 0 && ip < instructions.len() as i32 {
        let op = &instructions[ip as usize];
        match op {
            Hlf(reg) => {
                let r = regs.get(reg);
                *r /= 2;
                ip += 1;
            }
            Tpl(reg) => {
                let r = regs.get(reg);
                *r *= 3;
                ip += 1;
            }
            Inc(reg) => {
                let r = regs.get(reg);
                *r += 1;
                ip += 1;
            }
            Jmp(offset) => {
                ip += *offset;
            }
            Jie(reg, offset) => {
                let r = regs.get(reg);
                if *r % 2 == 0 {
                    ip += *offset;
                } else {
                    ip += 1;
                }
            }
            Jio(reg, offset) => {
                let r = regs.get(reg);
                if *r == 1 {
                    ip += *offset;
                } else {
                    ip += 1;
                }
            }
        }
    }

    regs
}
