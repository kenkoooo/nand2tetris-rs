use assembler::model::{Command, Operator, Place};

pub fn parse(line: &str) -> Result<Command, ()> {
    match line.trim().split("//").next() {
        Some(line) => {
            if line.is_empty() {
                Ok(Command::Comment)
            } else if &line[..1] == "@" {
                match &line[1..].parse::<i16>() {
                    Ok(a) => Ok(Command::Address(*a)),
                    Err(_) => Ok(Command::AddressSymbol(line[1..].to_owned())),
                }
            } else if &line[1..2] == "=" && line.len() == 3 {
                let s = line.chars().collect::<Vec<_>>();
                Place::parse(s[0])
                    .and_then(|dest| Place::parse(s[2]).map(|src| Command::Assign { dest, src }))
            } else if &line[1..2] == "=" && line.len() == 5 {
                let s = line.chars().collect::<Vec<_>>();

                let dest = Place::parse(s[0]);
                let left = Place::parse(s[2]);
                let operator = Operator::parse(s[3]);
                let right = Place::parse(s[4]);

                match (dest, left, operator, right) {
                    (Ok(dest), Ok(left), Ok(operator), Ok(right)) => Ok(Command::Operation {
                        dest,
                        left,
                        operator,
                        right,
                    }),
                    _ => Err(()),
                }
            } else {
                Err(())
            }
        }
        _ => unreachable!(),
    }
}
