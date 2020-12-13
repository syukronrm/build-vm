use nom::IResult;

use super::Token;
use crate::instruction::Opcode;
use nom::bytes::complete::take_while;
use nom::character::is_alphabetic;

pub fn opcode(i: &str) -> IResult<&str, Token> {
    let (i, opcode) = take_while(|c| is_alphabetic(c as u8))(i)?;
    Ok((
        i,
        Token::Op {
            code: Opcode::from(opcode),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode() {
        let result = opcode("load");
        assert_eq!(result.is_ok(), true);
        let (left, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(left, "");

        let result = opcode("loda");
        assert_eq!(result.is_ok(), true);
        let (left, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
        assert_eq!(left, "");
    }
}
