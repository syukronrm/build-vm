use super::Token;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    IResult,
};

// named!(pub register<&str, Token>,
//     do_parse!(
//         multispace0 >>
//         tag!("$") >>
//         reg_num: digit1 >>
//         (
//             Token::Register {
//                 reg_num: reg_num.parse::<u8>().unwrap()
//             }
//         )
//     )
// );

pub fn register(i: &str) -> IResult<&str, Token> {
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("$")(i)?;
    let (i, reg_num) = digit1(i)?;
    Ok((
        i,
        Token::Register {
            reg_num: reg_num.parse::<u8>().unwrap(),
        },
    ))
}

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
