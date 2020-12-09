use std::collections::HashMap;

use anyhow::Result;
use thiserror::Error;

pub struct Emulator {
    pub instructions: Vec<Instruction>,
    pub patch: Option<Patch>
}

impl Emulator {
    pub fn new(code: &String) -> Result<Self> {
        let emulator = Self {
            instructions: Self::parse_code(code)?,
            patch: None
        };
        Ok(emulator)
    }

    pub fn run(&self) -> EmulatorResult {
        let (mut ip, mut acc) = (0usize, 0i64);
        let already_visited= (0..self.instructions.len())
            .map(|i| (i, false))
            .collect::<HashMap<usize, bool>>();

        while let Some(instruction) = self.fetch_instruction(ip, &already_visited) {
            match instruction {
                Instruction::NOP(_value) => {
                    ip += 1;
                },
                Instruction::ACC(value) => {
                    match value {
                        Value::POS(n) => acc += *n as i64,
                        Value::NEG(n) => acc -= *n as i64
                    };
                    ip += 1;
                },
                Instruction::JMP(value) => {
                    match value {
                        Value::POS(n) => ip += n,
                        Value::NEG(n) => ip -= n
                    }
                }
            }
        }

        if !already_visited[&ip] {
            EmulatorResult::Terminated(acc)
        } else {
            EmulatorResult::EmergencyBreak(acc)
        }
    }

    fn fetch_instruction(&self, ip: usize, already_visited: &HashMap<usize, bool>) -> Option<&Instruction> {
        if already_visited[&ip] {
            None
        } else {
            already_visited[&ip] = true;
            if let Some(patch) = self.patch_check(ip) {
                Some(&patch)
            } else {
                self.instructions.get(ip)
            }
        }
    }

    fn patch_check(&self, ip: usize) -> Option<Instruction>{
        if let Some(patch) = self.patch {
            if patch.0 == ip {
                Some(patch.1)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_code(code: &String) -> Result<Vec<Instruction>> {
        code.lines().map(Self::parse_line).collect()
    }

    fn parse_line(lines: &str) -> Result<Instruction> {
        let mut iter = lines.split(" ");

        let instruction_str = iter.next().unwrap();
        let number_str = iter.next().unwrap();

        let instruction_value_prefix = number_str.chars().next().unwrap();
        let instruction_value_str = number_str.chars().skip(1).collect::<String>();

        let instruction_value = if instruction_value_prefix == '-' {
            Value::NEG(instruction_value_str.parse::<usize>().unwrap())
        } else {
            Value::POS(instruction_value_str.parse::<usize>().unwrap())
        };

        match instruction_str {
            "nop" => Ok(Instruction::NOP(instruction_value)),
            "acc" => Ok(Instruction::ACC(instruction_value)),
            "jmp" => Ok(Instruction::JMP(instruction_value)),
            instruction => Err(EmulatorError::UnknownInstruction(instruction.into()).into())
        }
    }
}

#[derive(Copy, Clone)]
pub enum Instruction {
    NOP(Value),
    ACC(Value),
    JMP(Value)
}

#[derive(Copy, Clone)]
enum Value {
    POS(usize),
    NEG(usize)
}

type Patch = (usize, Instruction);

#[derive(Copy, Clone)]
pub enum EmulatorResult {
    Terminated(i64),
    EmergencyBreak(i64)
}

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("Unknown instructio `{0}`.")]
    UnknownInstruction(String)
}