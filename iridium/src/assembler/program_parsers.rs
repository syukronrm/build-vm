use super::instruction_parsers::instruction_one;
use super::instruction_parsers::AssemblerInstruction;

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

//named!(pub program<&str, Program>,
//    do_parse!(
//        res: many_till!(instruction_one, eof!()) >>
//        (
//            Program {
//                instructions: res.0
//            }
//        )
//    )
//);

named!(pub program<&str, Program>,
    do_parse!(
        res: many1!(complete!(instruction_one)) >>
        (
            Program {
                instructions: res
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

    #[test]
    fn test_two_programs() {
        let result = program("load $0 #100\nload $1 #200\n");
        println!("{:#?}", result);
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "\n");
        assert_eq!(p.instructions.len(), 2);
    }
}
