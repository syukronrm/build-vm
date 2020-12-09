use super::opcode_parsers::*;
use super::operand_parsers::*;
use super::register_parsers::*;
use super::Token;
use nom::{character::complete::multispace0, IResult};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        match self.opcode {
            Token::Op { code } => match code {
                _ => {
                    bytes.push(code as u8);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        }

        for token in vec![&self.operand1, &self.operand2, &self.operand3] {
            match token {
                Some(t) => match t {
                    Token::Register { reg_num } => bytes.push(*reg_num),
                    Token::IntegerOperand { value } => {
                        let u16int = *value as u16;
                        let byte1 = u16int as u8;
                        let byte2 = (u16int >> 8) as u8;
                        bytes.push(byte2);
                        bytes.push(byte1);
                    }
                    _ => {
                        println!("Opcode found in operand field");
                        std::process::exit(1);
                    }
                },
                _ => {}
            }
        }

        bytes
    }
}

pub fn instruction_one(i: &str) -> IResult<&str, AssemblerInstruction> {
    let (i, _) = multispace0(i)?;
    let (i, o) = opcode_load(i)?;
    let (i, r) = register(i)?;
    let (i, i_) = integer_operand(i)?;
    let (i, _) = multispace0(i)?;
    Ok((
        i,
        AssemblerInstruction {
            opcode: o,
            operand1: Some(r),
            operand2: Some(i_),
            operand3: None,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one("load $0 #100");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
}
