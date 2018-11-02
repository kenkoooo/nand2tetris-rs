use elements::alu;
use elements::circuits::{ProgramCounter, Register};
use elements::gates;

pub struct CPU {
    data_register: Register,
    address_register: Register,
    pc: ProgramCounter,
    alu_out: [bool; 16],
}

impl CPU {
    pub fn calc(&mut self, input: [bool; 16], instruction: [bool; 16], reset: bool) {
        use self::gates::{and, not};
        let a_instruction_p = not(instruction[0]);
        let write_m = and(instruction[12], not(a_instruction_p));

        let new_a = gates::mux16(self.alu_out, instruction, a_instruction_p);
        let store_ap1
    }
}
