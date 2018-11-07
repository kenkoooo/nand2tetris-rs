use assembler::model::{Command, Operator, Place};

pub fn format_comp(left: Place, right: Place, operator: Operator) -> Result<&'static str, ()> {
    use self::Operator::*;
    use self::Place::*;
    match (left, operator, right) {
        (D, Plus, A) => Ok("0000010"),
        (A, Plus, D) => Ok("0000010"),
        (D, Plus, M) => Ok("1000010"),
        (M, Plus, D) => Ok("1000010"),
        (D, And, A) => Ok("0000000"),
        (D, And, M) => Ok("1000000"),
        (D, Or, A) => Ok("0010101"),
        (D, Or, M) => Ok("1010101"),
        (D, Plus, One) => Ok("0011111"),
        (A, Plus, One) => Ok("0110111"),
        (M, Plus, One) => Ok("1110111"),
        (D, Minus, A) => Ok("0010011"),
        (D, Minus, M) => Ok("1010011"),
        (M, Minus, D) => Ok("1000111"),
        (M, Minus, One) => Ok("1110010"),
        (A, Minus, One) => Ok("0110010"),
        (D, Minus, One) => Ok("0001110"),
        (Zero, Minus, One) => Ok("0111010"),
        _ => unimplemented!("{:?}", (left, operator, right)),
    }
}

pub fn format_address(x: i16) -> Result<String, ()> {
    if x < 0 {
        Err(())
    } else {
        let mut binary_representation = format!("{:b}", x).chars().rev().collect::<Vec<_>>();
        if binary_representation.len() > 15 {
            Err(())
        } else {
            while binary_representation.len() < 16 {
                binary_representation.push('0');
            }
            Ok(binary_representation.iter().rev().collect())
        }
    }
}

pub fn format_to_binary(cmd: &Command) -> Result<String, String> {
    match cmd {
        Command::Address(address) => format_address(*address),
        Command::Assign { dest, src } => {
            let dest = dest.format_dest();
            let src = src.format_single_comp();
            match (dest, src) {
                (Ok(dest), Ok(src)) => Ok("111".to_string() + src + dest + "000"),
                _ => Err(()),
            }
        }
        Command::Operation {
            dest,
            left,
            operator,
            right,
        } => {
            let dest = dest.format_dest();
            let comp = format_comp(*left, *right, *operator);
            match (dest, comp) {
                (Ok(dest), Ok(comp)) => Ok("111".to_string() + comp + dest + "000"),
                _ => Err(()),
            }
        }
        Command::Jump { dest, jump } => {
            let comp = dest.format_single_comp();
            let jump = jump.format_jump();
            match comp {
                Ok(comp) => Ok("111".to_string() + comp + "000" + jump),
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
    .map_err(|_| format!("Format error: {:?}", cmd))
}
