use vm::model::{Segment, VMCommand};

pub fn translate(commands: &Vec<VMCommand>, file_label: &str) -> Result<Vec<String>, String> {
    let mut writer = VirtualMachineWriter {
        output: vec![],
        file_label: file_label.to_string(),
    };
    writer.add_str("@256");
    writer.add_str("D=A");
    writer.add_str("@SP");
    writer.add_str("M=D");

    for &command in commands.iter() {
        match command {
            VMCommand::Push(segment, index) => writer.push(segment, index),
            VMCommand::Pop(segment, index) => writer.pop(segment, index),
            VMCommand::EQ | VMCommand::LT | VMCommand::GT => {
                writer.get_2_numbers_from_stack();
                match command {
                    VMCommand::EQ => {
                        let equal_label = writer.generate_label("equal_label");
                        let finish_label = writer.generate_label("finish_label");

                        // if D!=M, jump to equal_label and D will be 0
                        writer.add_str("D=M-D");
                        writer.add_str(&format!("@{}", equal_label));
                        writer.add_str("D;JNE"); // if D!=0, jump to equal_label

                        // if D==M, D will be -1 and jump to finish_label
                        writer.add_str("D=-1");
                        writer.add_str(&format!("@{}", finish_label));
                        writer.add_str("0;JMP"); // jump to finish_label

                        writer.add_str(&format!("({})", equal_label));
                        writer.add_str("D=0");

                        writer.add_str(&format!("({})", finish_label));
                    }
                    VMCommand::LT => {
                        // true if D > M
                        let larger_label = writer.generate_label("larger_label");
                        let finish_label = writer.generate_label("finish_label");

                        // if , jump to larger_label and D will be 0
                        writer.add_str("D=D-M");
                        writer.add_str(&format!("@{}", larger_label));
                        writer.add_str("D;JGT"); // if D>0, jump to larger_label

                        // if D<=M, D will be 0 and jump to finish_label
                        writer.add_str("D=0");
                        writer.add_str(&format!("@{}", finish_label));
                        writer.add_str("0;JMP"); // jump to finish_label

                        writer.add_str(&format!("({})", larger_label));
                        writer.add_str("D=-1");

                        writer.add_str(&format!("({})", finish_label));
                    }
                    VMCommand::GT => {
                        // true if M > D
                        let smaller_label = writer.generate_label("smaller_label");
                        let finish_label = writer.generate_label("finish_label");

                        // if , jump to smaller_label and D will be 0
                        writer.add_str("D=M-D");
                        writer.add_str(&format!("@{}", smaller_label));
                        writer.add_str("D;JGT"); // if D>0, jump to smaller_label

                        // if D>=M, D will be 0 and jump to finish_label
                        writer.add_str("D=0");
                        writer.add_str(&format!("@{}", finish_label));
                        writer.add_str("0;JMP"); // jump to finish_label

                        writer.add_str(&format!("({})", smaller_label));
                        writer.add_str("D=-1");

                        writer.add_str(&format!("({})", finish_label));
                    }
                    _ => unreachable!(),
                }
                writer.remove_2_and_push_data();
            }
            VMCommand::Add | VMCommand::Sub | VMCommand::Or | VMCommand::And => {
                writer.get_2_numbers_from_stack();
                match command {
                    VMCommand::Add => writer.add_str("D=D+M"),
                    VMCommand::Sub => writer.add_str("D=M-D"),
                    VMCommand::And => writer.add_str("D=M&D"),
                    VMCommand::Or => writer.add_str("D=M|D"),
                    _ => unreachable!(),
                }
                writer.remove_2_and_push_data();
            }
            VMCommand::Neg | VMCommand::Not => {
                writer.add_str("@SP");
                writer.add_str("A=M");
                writer.add_str("A=A-1");
                match command {
                    VMCommand::Neg => writer.add_str("M=-M"),
                    VMCommand::Not => writer.add_str("M=!M"),
                    _ => unreachable!(),
                }
                writer.add_str("D=A+1");
                writer.add_str("@SP");
                writer.add_str("M=D");
            }
            VMCommand::Comment => {}
        }
    }
    Ok(writer.output)
}

struct VirtualMachineWriter {
    output: Vec<String>,
    file_label: String,
}

impl VirtualMachineWriter {
    fn add_str(&mut self, s: &str) {
        self.output.push(s.to_owned());
    }

    fn push(&mut self, segment: Segment, index: usize) {
        match segment {
            Segment::Constant => {
                self.add_str(&format!("@{}", index));
                self.add_str("D=A");
                self.add_str("@SP");
                self.add_str("A=M");
                self.add_str("M=D");
                self.add_str("A=A+1");
                self.add_str("D=A");
                self.add_str("@SP");
                self.add_str("M=D");
            }
            _ => unimplemented!("{:?}", segment),
        }
    }

    fn pop(&mut self, segment: Segment, index: usize) {
        // top
        self.add_str("@SP");
        self.add_str("A=M-1");
        self.add_str("D=M");

        match segment {
            Segment::Static => {
                let static_label = format!("@{}.{}", self.file_label, index);
                self.add_str(&static_label);
                self.add_str("M=D");
            }
            _ => unimplemented!("{:?}", segment),
        }

        // pop
        self.add_str("@SP");
        self.add_str("M=M-1");
        unimplemented!()
    }

    fn generate_label(&self, label: &str) -> String {
        format!("{}{}", label, self.output.len())
    }

    /// Get 2 numbers from stack and 1st one will be D, and 2nd will be M.
    fn get_2_numbers_from_stack(&mut self) {
        self.add_str("@SP");
        self.add_str("A=M");
        self.add_str("A=A-1");
        self.add_str("D=M");
        self.add_str("A=A-1");
    }

    fn remove_2_and_push_data(&mut self) {
        self.add_str("@SP");
        self.add_str("A=M");
        self.add_str("A=A-1");
        self.add_str("A=A-1");
        self.add_str("M=D");
        self.add_str("D=A+1");
        self.add_str("@SP");
        self.add_str("M=D");
    }
}
