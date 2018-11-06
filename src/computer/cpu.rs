use elements::alu;
use elements::circuits::{ProgramCounter, Register};
use elements::gates;

pub struct CPU {
    data_register: Register,
    address_register: Register,
    pc: ProgramCounter,

    a_out: [bool; 16],
    d_out: [bool; 16],
}

fn mux_alu(
    a_out: [bool; 16],
    input: [bool; 16],
    instruction: [bool; 16],
    d_out: [bool; 16],
) -> ([bool; 16], bool, bool) {
    let a_m_out = gates::mux16(a_out, input, instruction[12]);
    alu::alu(
        d_out,
        a_m_out,
        instruction[11],
        instruction[10],
        instruction[9],
        instruction[8],
        instruction[7],
        instruction[6],
    )
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            data_register: Register::new(),
            address_register: Register::new(),
            pc: ProgramCounter::new(),

            a_out: [false; 16],
            d_out: [false; 16],
        }
    }

    pub fn tick(&mut self, input: [bool; 16], instruction: [bool; 16], reset: bool) {
        use self::gates::{and, not, or};

        let (alu_out, zr_out, ng_out) = mux_alu(self.a_out, input, instruction, self.d_out);

        let a_instruction = not(instruction[15]);
        let c_instruction = not(a_instruction);
        let alu_to_a = and(c_instruction, instruction[5]);
        let a_reg_in = gates::mux16(instruction, alu_out, alu_to_a);
        let load_a = or(a_instruction, alu_to_a);
        self.address_register.tick(a_reg_in, load_a);

        let load_d = and(c_instruction, instruction[4]);
        self.data_register.tick(alu_out, load_d);

        let jeq = and(zr_out, instruction[1]);
        let jlt = and(ng_out, instruction[2]);
        let zero_or_neg = or(zr_out, ng_out);
        let positive = not(zero_or_neg);
        let jgt = and(positive, instruction[0]);
        let jle = or(jeq, jlt);
        let jump_to_a = or(jle, jgt);
        let pc_load = and(c_instruction, jump_to_a);
        let pc_inc = not(pc_load);
        self.pc.tick(self.a_out, reset, pc_load, pc_inc);
    }

    pub fn tock(
        &mut self,
        input: [bool; 16],
        instruction: [bool; 16],
    ) -> ([bool; 16], bool, [bool; 15], [bool; 15]) {
        use self::gates::{and, not};

        let d_out = self.data_register.tock();
        let a_out = self.address_register.tock();
        let (out_m, _, _) = mux_alu(a_out, input, instruction, d_out);
        let pc = self.pc.tock();
        let pc = [
            pc[0], pc[1], pc[2], pc[3], pc[4], pc[5], pc[6], pc[7], pc[8], pc[9], pc[10], pc[11],
            pc[12], pc[13], pc[14],
        ];
        let address_m = [
            a_out[0], a_out[1], a_out[2], a_out[3], a_out[4], a_out[5], a_out[6], a_out[7],
            a_out[8], a_out[9], a_out[10], a_out[11], a_out[12], a_out[13], a_out[14],
        ];
        let a_instruction = not(instruction[15]);
        let c_instruction = not(a_instruction);
        let write_m = and(c_instruction, instruction[3]);

        self.a_out = a_out;
        self.d_out = d_out;

        (out_m, write_m, address_m, pc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tools;

    fn flat(x: [bool; 16]) -> usize {
        x.iter()
            .enumerate()
            .fold(0, |acc, (i, &b)| if b { acc + (1 << i) } else { acc })
    }

    fn convert16(x: i16) -> [bool; 16] {
        let mut result = [false; 16];
        for i in 0..15 {
            result[i] = (1 << i) & x != 0;
        }
        result[15] = x < 0;
        result
    }

    fn convert16_str(x: &str) -> [bool; 16] {
        let mut res = [false; 16];
        for i in 0..16 {
            res[i] = &x[(15 - i)..(15 - i + 1)] == "1";
        }
        res
    }

    fn bit_16_to_15(x: [bool; 16]) -> [bool; 15] {
        let mut res = [false; 15];
        for i in 0..15 {
            res[i] = x[i];
        }
        res
    }

    #[test]
    fn cpu_test() {
        let t = tools::read_test_data("tests/05/CPU.cmp").unwrap();
        let mut iter = t[1..].iter().map(|t| {
            println!("{:?}", t);
            let is_set = t[1].chars().next_back().unwrap() == '+';
            let input = convert16(t[2].parse::<i16>().unwrap());
            let instruction = convert16_str(&t[3]);
            let reset = t[4] == "1";
            let out = t[5].parse::<i16>().ok().map(|v| convert16(v));
            let write_m = t[6] == "1";
            let address = convert16(t[7].parse::<i16>().unwrap());
            let pc = convert16(t[8].parse::<i16>().unwrap());

            (
                is_set,
                input,
                instruction,
                reset,
                out,
                write_m,
                bit_16_to_15(address),
                bit_16_to_15(pc),
            )
        });

        let mut cpu = CPU::new();

        while let Some((is_set, input, instruction, reset, _, _, _, _)) = iter.next() {
            assert!(is_set);
            cpu.tick(input, instruction, reset);
            let (_, _, _, _, t_out, t_write_m, t_address, t_pc) = iter.next().unwrap();
            let (out, write_m, address, pc) = cpu.tock(input, instruction);
            assert_eq!(write_m, t_write_m);
            assert_eq!(address, t_address);
            assert_eq!(pc, t_pc);
            if let Some(t_out) = t_out {
                assert_eq!(out, t_out, "{} {}", flat(out), flat(t_out));
            }
        }
    }
}
