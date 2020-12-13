use nom::{branch::alt, combinator::eof, multi::many_till, IResult};

use super::instruction_parsers::*;

#[derive(Debug)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for instruction in &self.instructions {
            bytes.append(&mut instruction.to_bytes());
        }
        bytes
    }
}

pub fn program(i: &str) -> IResult<&str, Program> {
    let (i, (instructions, _)) = many_till(
        alt((instruction_one, instruction_two, instruction_three)),
        eof,
    )(i)?;
    Ok((i, Program { instructions }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let result = program("load $0 #100\n");
        println!("{:#?}", result);
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(p.instructions.len(), 1);
    }

    #[test]
    fn test_two_programs() {
        let result = program("load $0 #100\nload $1 #200\n");
        println!("{:#?}", result);
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(p.instructions.len(), 2);
    }
}
