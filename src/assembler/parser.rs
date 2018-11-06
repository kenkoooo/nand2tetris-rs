use assembler::model::{Command, Operator, Place};

pub fn parse(code: &str) -> Result<Vec<Command>, ()> {
    code.trim()
        .split('\n')
        .map(|s| s.split("//").next().unwrap())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            if &s[..1] == "@" {
                match &s[1..].parse::<i16>() {
                    Ok(a) => Ok(Command::Address(*a)),
                    Err(_) => Ok(Command::AddressSymbol(s[1..].to_owned())),
                }
            } else if &s[1..2] == "=" && s.len() == 3 {
                let s = s.chars().collect::<Vec<_>>();
                Place::parse(s[0])
                    .and_then(|dest| Place::parse(s[2]).map(|src| Command::Assign { dest, src }))
            } else if &s[1..2] == "=" && s.len() == 5 {
                let s = s.chars().collect::<Vec<_>>();

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
        })
        .collect::<Result<Vec<_>, ()>>()
}
