#![allow(dead_code)]

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum VMError { //list of all errors for the stack, 
    UNDERFLOW,
}

#[derive(Debug, Default)]
struct Word(u32); //stack elements are 256 bit values, so use 32 octets

#[derive(Default)]
struct Stack {
    pub values: Vec<Word>, //values is a variable-sized Vec
}

// instruction class
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction { // the different instructions to be executed
    Add(), 
    Push1(u8),
}

fn execute(stack: &mut Stack, instruction: &Instruction) -> Result<(), VMError> { //instruction is immutable. So consider dereferencing the borrow as &
    match instruction {
        Instruction::Add() => {
            let a = stack.values.pop().ok_or(VMError::UNDERFLOW)?; //if there is error in popping, then it is an underflow, so handle it.
            let b = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
            stack.values.push(Word(a.0 + b.0));
        }
        Instruction::Push1(value) => stack.values.push(Word(*value as u32)), //casting `&u8` as `u32` is invalid, so dereference the expression: `*value
    }
    Ok(())
}

fn playground() -> Result<(), VMError> { //Result is a type that represents either success (Ok) or failure (Err)
    let mut stack = Stack::default(); //Default value is []. i.e., empty

    let instructions = vec![ //The instructions are push 1 and push 2, then add
        Instruction::Push1(1), //Instruction is an Enum
        Instruction::Push1(2),
        Instruction::Add(),
    ];

    for instruction in instructions.iter() { // for each instruction
        execute(&mut stack, instruction)?; //insead of taking the return to any variable, you can use the ? to check if the return is a success or error
    }
    Ok(()) //return  NULL tuple Ok(()) if successful
}

fn main() {
    match playground() {
        Ok(()) => println!("Done!"), //if all the instructions are executed without errors
        Err(error) => println!("Error: {:?}", error),
    }
}