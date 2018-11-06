use assembler::model::{Command, Operator, Place};

pub fn format_comp(left: Place, right: Place, operator: Operator) -> Result<&'static str, ()> {
    use self::Operator::*;
    use self::Place::*;
    match (left, operator, right) {
        (D, Plus, A) | (A, Plus, D) => Ok("0000010"),
        (D, Minus, A) => Ok("0010011"),
        _ => unimplemented!(),
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

pub fn format_to_binary(cmd: &Command) -> Result<String, ()> {
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
        _ => unreachable!(),
    }
}
