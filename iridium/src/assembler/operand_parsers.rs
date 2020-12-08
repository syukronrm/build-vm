use super::Token;
use nom::character::complete::digit1;

named!(integer_operand<&str, Token>,
    do_parse!(
        take_while!(|c| c == ' ' || c == '\t') >>
        tag!("#") >>
        operand: digit1 >>
        (
            Token::IntegerOperand {
                value: operand.parse::<i32>().unwrap()
            }
        )
    )
);

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
