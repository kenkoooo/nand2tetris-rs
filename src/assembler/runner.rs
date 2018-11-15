use assembler::model::{Command, Jump, Operator, Place};

const SIZE32K: usize = 1024 * 32;

pub struct Runner {
    pub memory: [i16; SIZE32K],
    pub address: i16,
    pub data: i16,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            memory: [0; SIZE32K],
            data: 0,
            address: 0,
        }
    }
    pub fn run(&mut self, commands: &Vec<Command>) {
        let mut pc = 0;
        while pc < commands.len() {
            use self::Command::{Address, Assign, Operation};
            match commands[pc] {
                Address(a) => {
                    self.address = a;
                    pc += 1;
                }

                Assign { src, dest } => {
                    match (src, dest) {
                        (Place::A, Place::D) => self.data = self.address,
                        (Place::D, Place::A) => self.address = self.data,
                        (Place::Zero, Place::D) => self.data = 0,
                        (Place::D, Place::M) => {
                            assert!(self.address >= 0);
                            self.memory[self.address as usize] = self.data;
                        }
                        (Place::M, Place::A) => {
                            assert!(self.address >= 0);
                            self.address = self.memory[self.address as usize];
                        }
                        (Place::M, Place::D) => {
                            assert!(self.address >= 0);
                            self.data = self.memory[self.address as usize];
                        }
                        (Place::NotM, Place::M) => {
                            assert!(self.address >= 0);
                            self.memory[self.address as usize] =
                                !self.memory[self.address as usize];
                        }
                        _ => unimplemented!("{:?}", commands[pc]),
                    }
                    pc += 1;
                }

                Operation {
                    dest,
                    left,
                    operator,
                    right,
                } => {
                    use self::Operator::*;
                    let result = match (left, operator, right) {
                        (Place::A, Plus, Place::One) => self.address + 1,
                        (Place::A, Minus, Place::One) => self.address - 1,
                        (Place::D, Plus, Place::M) => {
                            assert!(self.address >= 0);
                            self.data + self.memory[self.address as usize]
                        }
                        (Place::M, Minus, Place::D) => {
                            assert!(self.address >= 0);
                            self.memory[self.address as usize] - self.data
                        }
                        (Place::D, Minus, Place::M) => {
                            assert!(self.address >= 0);
                            self.data - self.memory[self.address as usize]
                        }
                        (Place::Zero, Minus, Place::One) => -1,
                        (Place::Zero, Minus, Place::M) => {
                            assert!(self.address >= 0);
                            -self.memory[self.address as usize]
                        }
                        (Place::M, And, Place::D) => {
                            assert!(self.address >= 0);
                            self.memory[self.address as usize] & self.data
                        }
                        (Place::M, Or, Place::D) => {
                            assert!(self.address >= 0);
                            self.memory[self.address as usize] | self.data
                        }
                        _ => unimplemented!("{:?}", commands[pc]),
                    };
                    match dest {
                        Place::A => self.address = result,
                        Place::D => self.data = result,
                        Place::M => {
                            assert!(self.address >= 0);
                            self.memory[self.address as usize] = result;
                        }
                        _ => unimplemented!("{:?}", dest),
                    }
                    pc += 1;
                }
                Command::Jump { dest, jump } => {
                    let dest = match dest {
                        Place::Zero => 0,
                        Place::D => self.data,
                        Place::A => self.address,
                        _ => unimplemented!("{:?}", dest),
                    };

                    let is_jump = match jump {
                        Jump::JNE => dest != 0,
                        Jump::JGT => dest > 0,
                        Jump::JMP => true,
                        _ => unimplemented!("{:?}", jump),
                    };
                    if is_jump {
                        assert!(self.address >= 0);
                        pc = self.address as usize;
                    } else {
                        pc += 1;
                    }
                }
                _ => unreachable!("{:?}", commands[pc]),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assembler::{optimizer, parser};
    use tools;

    #[test]
    fn runner_simple_add_test() {
        let commands = tools::read_file("tests/07/StackArithmetic/SimpleAdd/SimpleAdd.asm")
            .map_err(|_| "IO Error".to_string())
            .and_then(|content| {
                content
                    .trim()
                    .split('\n')
                    .map(|line| parser::parse(line))
                    .collect::<Result<Vec<_>, _>>()
            })
            .map(|commands| {
                optimizer::optimize(&commands)
                    .iter()
                    .map(|cmd| cmd.clone().unwrap())
                    .collect::<Vec<_>>()
            })
            .unwrap();

        let mut runner = Runner::new();
        runner.run(&commands);
        assert_eq!(runner.memory[0], 257);
        assert_eq!(runner.memory[256], 15);
    }

    #[test]
    fn runner_stack_test() {
        let commands = tools::read_file("tests/07/StackArithmetic/StackTest/StackTest.asm")
            .map_err(|_| "IO Error".to_string())
            .and_then(|content| {
                content
                    .trim()
                    .split('\n')
                    .map(|line| parser::parse(line))
                    .collect::<Result<Vec<_>, _>>()
            })
            .map(|commands| {
                optimizer::optimize(&commands)
                    .iter()
                    .map(|cmd| cmd.clone().unwrap())
                    .collect::<Vec<_>>()
            })
            .unwrap();

        let mut runner = Runner::new();
        runner.run(&commands);

        assert_eq!(runner.memory[0], 266);
        assert_eq!(runner.memory[256], -1);
        assert_eq!(runner.memory[257], 0);
        assert_eq!(runner.memory[258], 0);
        assert_eq!(runner.memory[259], 0);
        assert_eq!(runner.memory[260], -1);
        assert_eq!(runner.memory[261], 0);
        assert_eq!(runner.memory[262], -1);
        assert_eq!(runner.memory[263], 0);
        assert_eq!(runner.memory[264], 0);
        assert_eq!(runner.memory[265], -91);
    }
}
