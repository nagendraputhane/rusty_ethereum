#![allow(dead_code)]

#[derive (Debug, Copy, Clone, PartialEq, Eq)]
enum VMError {
    UNDERFLOW,
}

#[derive(Default)]
struct Word(u32); // we need a type for 256 bit words

#[derive(Default)]
struct Stack {
    pub values: Vec<Word>, // variable sized vector
}
//if you wnt to put methods inside them, use impl

    //pub fn add (&mut self) -> Result<(), VMError>{
        //instead of returning nothing for add, 
        //we can return nothing if its a success, -< () zero sized tuble
        //else return an error

        //access to inside variable
        // & : reference, &mut : mutable reference
        // things are immutable by default

        // pub fn pop (&mut self) -> Option<T> // It gives a value T or not
        // so we should decide what to do if value is not returned
        //let a = self.values.pop().ok_or(VMError::UNDERFLOW)?; 
        //? : if its a success case, return it. Or return the error
        //let b = self.values.pop().ok_or(VMError::UNDERFLOW)?;
        //self.value.push(Word(a.0 + b.0));
        //Ok(()) //return success
    //}

// instruction class
#[derive (Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Add(), // Add is parameter with no parameters
    Push1(u8), //named tuple that takes u8
}

fn execute(stack: &mut Stack, instruction: &Instruction) -> Result<(), VMError> {
    println!("{:?}", instruction);
    match instruction {
        Instruction::Add() => {
            let a = stack.values.pop().ok_or(VMError::UNDERFLOW)?; 
            let b = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
            stack.values.push(Word(a.0 + b.0));
        }
        Instruction::Push1(value) => stack.values.push(Word(*value as u32)),
    }
    Ok(())
}

fn playground () -> Result<(), VMError> {
    let mut stack = Stack::default();
    // we made a stack
    // next create a instruction vecotr with psuh1, push1, add
    let instructions = vec![
        Instruction::Push1(1),
        Instruction::Push1(2),
        Instruction::Add(),
    ];

    //pop in a loop
    // while instruction is not empty, keep popping
    for instruction in instructions.iter() {
        execute (&mut stack, instruction)?;
    }
    Ok(())
}

fn main() {
    match playground() {
        Ok(()) => println!("Done!"),
        Err(error) => println!("Error: {:?}", error),
    }
}

// MVP
// Take an instruction input buff
// pop in a loop
// execute

// Add 0x01
// Push1 0x60