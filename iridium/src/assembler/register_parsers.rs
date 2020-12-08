use super::Token;
use nom::character::complete::digit1;

named!(register<&str, Token>,
    do_parse!(
        take_while!(|c| c == ' ' || c == '\t') >>
        tag!("$") >>
        reg_num: digit1 >>
        (
            Token::Register {
                reg_num: reg_num.parse::<u8>().unwrap()
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let result = register("$10");
        assert_eq!(result.is_ok(), true);
        let result = register("10");
        assert_eq!(result.is_ok(), false);
        let result = register("$a");
        assert_eq!(result.is_ok(), false);
        let result = register(" $10");
        assert_eq!(result.is_ok(), true);
    }
}
