use super::vm::VM;
use std::{
    io::{self, Write},
    num::ParseIntError,
};
use crate::assembler::program_parsers::program;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridium!");
        loop {
            let mut buffer = String::new();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            io::stdin()
                .read_line(&mut buffer)
                .expect("Unable to read line from stdin");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                }
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                }
                ".registers" => {
                    println!("Listing registers:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of regiter listing")
                }
                _ => {
                    println!("{:#?}", buffer);
                    let parsed_program = program(buffer);
                    if !parsed_program.is_ok() {
                        println!("Unable to parse input");
                        continue;
                    }

                    let (_, result) = parsed_program.unwrap();
                    let bytecodes = result.to_bytes();

                    for byte in bytecodes {
                        self.vm.add_byte(byte);
                    }

//                     let result = self.parse_hex(buffer);
//                    match result {
//                        Ok(bytes) => {
//                            self.vm.program.append(&mut bytes.clone());
//                        }
//                        Err(_) => {
//                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.");
//                        }
//                    }
                    self.vm.run_once();
                }
            }
        }
    }

    fn parse_hex(&self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split: Vec<&str> = i.split(' ').collect();
        let mut programs = vec![];
        for s in split {
            let byte = u8::from_str_radix(s, 16);
            match byte {
                Ok(b) => programs.push(b),
                Err(err) => return Err(err),
            }
        }
        Ok(programs)
    }
}
