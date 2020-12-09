use super::instruction_parsers::instruction_one;
use super::instruction_parsers::AssemblerInstruction;

#[derive(Debug)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

named!(pub program<&str, Program>,
    do_parse!(
        instructions: many1!(instruction_one) >>
        (
            Program {
                instructions
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let result = program("load $0 #100\n");
        println!("{:#?}", result);
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "\n");
        assert_eq!(p.instructions.len(), 1);
    }
}
