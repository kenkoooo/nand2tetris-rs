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
    Jump {
        dest: Place,
        jump: Jump,
    },
    Comment,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Place {
    A,
    D,
    M,
    One,
    Zero,
    MinusOne,
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
            Place::Zero => Ok("0101010"),
            _ => Err(()),
        }
    }

    pub fn parse(x: &str) -> Result<Place, ()> {
        match x {
            "A" => Ok(Place::A),
            "M" => Ok(Place::M),
            "D" => Ok(Place::D),
            "1" => Ok(Place::One),
            "-1" => Ok(Place::MinusOne),
            "0" => Ok(Place::Zero),
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
    pub fn parse(x: &str) -> Result<Operator, ()> {
        match x {
            "+" => Ok(Operator::Plus),
            "-" => Ok(Operator::Minus),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl Jump {
    pub fn parse(x: &str) -> Result<Jump, ()> {
        match x {
            "JGT" => Ok(Jump::JGT),
            "JEQ" => Ok(Jump::JEQ),
            "JGE" => Ok(Jump::JGE),
            "JLT" => Ok(Jump::JLT),
            "JNE" => Ok(Jump::JNE),
            "JLE" => Ok(Jump::JLE),
            "JMP" => Ok(Jump::JMP),
            _ => Err(()),
        }
    }

    pub fn format_jump(&self) -> &'static str {
        use self::Jump::*;
        match self {
            JGT => "001",
            JEQ => "010",
            JGE => "011",
            JLT => "100",
            JNE => "101",
            JLE => "110",
            JMP => "111",
        }
    }
}
