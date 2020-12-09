use super::Token;
use nom::bytes::complete::tag;
use nom::{
    character::complete::{digit1, multispace0},
    IResult,
};

pub fn integer_operand(i: &str) -> IResult<&str, Token> {
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("#")(i)?;
    let (i, operand) = digit1(i)?;
    Ok((
        i,
        Token::IntegerOperand {
            value: operand.parse::<i32>().unwrap(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_operand() {
        let result = integer_operand("#10");
        assert_eq!(result.is_ok(), true);
        let result = integer_operand("10");
        assert_eq!(result.is_ok(), false);
        let result = integer_operand("#a");
        assert_eq!(result.is_ok(), false);
        let result = integer_operand(" #10");
        assert_eq!(result.is_ok(), true);
    }
}
