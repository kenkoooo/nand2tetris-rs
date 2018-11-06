#[derive(Debug, PartialEq)]
pub enum Command {
    Address(i16),
    AddressSymbol(String),
    Assign {
        dest: Place,
        src: Place,
    },
    Operation {
        dest: Place,
        left: Place,
        operator: Operator,
        right: Place,
    },
    Comment,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Place {
    A,
    D,
    M,
    One,
}

impl Place {
    pub fn format_dest(&self) -> Result<&'static str, ()> {
        match self {
            Place::A => Ok("100"),
            Place::D => Ok("010"),
            Place::M => Ok("001"),
            _ => Err(()),
        }
    }

    pub fn format_single_comp(&self) -> Result<&'static str, ()> {
        match self {
            Place::A => Ok("0110000"),
            Place::M => Ok("1110000"),
            Place::D => Ok("0001100"),
            _ => Err(()),
        }
    }

    pub fn parse(x: char) -> Result<Place, ()> {
        match x {
            'A' => Ok(Place::A),
            'M' => Ok(Place::M),
            'D' => Ok(Place::D),
            '1' => Ok(Place::One),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
}

impl Operator {
    pub fn parse(x: char) -> Result<Operator, ()> {
        match x {
            '+' => Ok(Operator::Plus),
            '-' => Ok(Operator::Minus),
            _ => Err(()),
        }
    }
}
