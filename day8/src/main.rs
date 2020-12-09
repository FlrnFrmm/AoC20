mod emulator;

use anyhow::Result;
use std::io::Read;

use emulator::*;

fn main() -> Result<()> {
    let instructions = parse_input()?;

    println!("Result one: {}", evaluate_instructions(&instructions)?);

    println!("Result two: {}", acc_for_termination(&instructions)?);

    Ok(())
}

fn evaluate_instructions(instructions_as_str: &String) -> Result<i64> {
    let emulator = emulator::Emulator::new(instructions_as_str)?;
    match emulator.run() {
        emulator::EmulatorResult::Terminated(acc_value) => eprint!("Terminated with {}", acc_value),
        emulator::EmulatorResult::EmergencyBreak(acc_value) => Ok(acc_value)
    }
}

fn acc_for_termination(instructions_as_str: &String) -> Result<i64> {
    let mut emulator = emulator::Emulator::new(instructions_as_str)?;

    for (index, instruction) in emulator.instructions.iter().enumerate() {
        match instruction {
            Instruction::JMP(value) => {
                emulator.patch = Some((index, Instruction::NOP(*value)));
                match emulator.run() {
                    emulator::EmulatorResult::Terminated(acc_value) => return Ok(acc_value),
                    _ => { }
                }
            },
            Instruction::NOP(value) => {
                emulator.patch = Some((index, Instruction::JMP(*value)));
                match emulator.run() {
                    emulator::EmulatorResult::Terminated(acc_value) => return Ok(acc_value),
                    _ => { }
                }
            },
            _ => {} 
        };

    }

    eprint!("No terminating code change found.")
}

fn parse_input() -> Result<String> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input)
}
