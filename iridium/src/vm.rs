use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            Opcode::JMP => {
                let register = self.registers[self.next_8_bits() as usize];
                self.pc = register as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 == register2;
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            _ => {
                println!("Uncovered instructin! Terminating!");
                return true;
            }
        }
        false
    }

    pub fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    pub fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1] as u16);
        self.pc += 2;
        return result;
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1)
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1)
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 1, 244];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500)
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![1, 0, 1, 2];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![2, 1, 0, 2];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![3, 1, 0, 2];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![4, 1, 0, 2];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 2);
        assert_eq!(test_vm.remainder, 0);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![6, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 5);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 1, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![8, 0, 0, 0, 1, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.registers[1] = 1;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 2;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 8);
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = get_test_vm();
        test_vm.equal_flag = false;
        test_vm.program = vec![10, 0, 10, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);

        test_vm.registers[0] = 5;
        test_vm.equal_flag = true;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 5);
    }
}
