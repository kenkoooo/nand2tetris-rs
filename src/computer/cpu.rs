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
    pub fn new() -> Self {
        CPU {
            data_register: Register::new(),
            address_register: Register::new(),
            pc: ProgramCounter::new(),
            alu_out: [false; 16],
        }
    }
    pub fn calc(
        &mut self,
        input: [bool; 16],
        instruction: [bool; 16],
        reset: bool,
    ) -> ([bool; 16], bool, [bool; 15], [bool; 15]) {
        use self::gates::{and, not, or};
        // Not(in=instruction[15], out=Ainstruction);
        let a_instruction = not(instruction[15]);

        // Not(in=Ainstruction, out=Cinstruction);
        let c_instruction = not(a_instruction);

        // And(a=Cinstruction, b=instruction[5], out=ALUtoA);    // C-inst and dest to A-reg?
        let alu_to_a = and(c_instruction, instruction[5]);

        // Mux16(a=instruction, b=ALUout, sel=ALUtoA, out=Aregin);
        let a_reg_in = gates::mux16(instruction, self.alu_out, alu_to_a);

        // Or(a=Ainstruction, b=ALUtoA, out=loadA);    // load A if A-inst or C-inst&dest to A-reg
        let load_a = or(a_instruction, alu_to_a);

        // ARegister(in=Aregin, load=loadA, out=Aout);
        self.address_register.tick(a_reg_in, load_a);
        let a_out = self.address_register.tock();

        // Mux16(a=Aout, b=inM, sel=instruction[12], out=AMout);   // select A or M based on a-bit
        let a_m_out = gates::mux16(a_out, input, instruction[12]);

        // And(a=Cinstruction, b=instruction[4], out=loadD);
        let load_d = and(c_instruction, instruction[4]);

        // DRegister(in=ALUout, load=loadD, out=Dout);    // load the D register from ALU
        self.data_register.tick(self.alu_out, load_d);
        println!("alu_out{}", flat(self.alu_out));
        let d_out = self.data_register.tock();

        // ALU(x=Dout, y=AMout, zx=instruction[11], nx=instruction[10],
        //     zy=instruction[9], ny=instruction[8], f=instruction[7],
        //     no=instruction[6], out=ALUout, zr=ZRout, ng=NGout); // calculate
        let (out_m, zr_out, ng_out) = alu::alu(
            d_out,
            a_m_out,
            instruction[11],
            instruction[10],
            instruction[9],
            instruction[8],
            instruction[7],
            instruction[6],
        );
        self.alu_out = out_m;

        // // Set outputs for writing memory
        // Or16(a=false, b=Aout, out[0..14]=addressM);
        // Or16(a=false, b=ALUout, out=outM);
        // And(a=Cinstruction, b=instruction[3], out=writeM);
        let address_m = [
            a_out[0], a_out[1], a_out[2], a_out[3], a_out[4], a_out[5], a_out[6], a_out[7],
            a_out[8], a_out[9], a_out[10], a_out[11], a_out[12], a_out[13], a_out[14],
        ];
        let write_m = and(c_instruction, instruction[3]);

        // // calc PCload & PCinc - whether to load PC with A reg
        // And(a=ZRout, b=instruction[1], out=jeq);    // is zero and jump if zero
        let jeq = and(zr_out, instruction[1]);
        // And(a=NGout, b=instruction[2], out=jlt);    // is neg and jump if neg
        let jlt = and(ng_out, instruction[2]);
        // Or(a=ZRout, b=NGout, out=zeroOrNeg);
        let zero_or_neg = or(zr_out, ng_out);
        // Not(in=zeroOrNeg, out=positive);            // is positive (not zero and not neg)
        let positive = not(zero_or_neg);
        // And(a=positive, b=instruction[0], out=jgt); // is pos and jump if pos
        let jgt = and(positive, instruction[0]);
        // Or(a=jeq, b=jlt, out=jle);
        let jle = or(jeq, jlt);
        // Or(a=jle, b=jgt, out=jumpToA);              // load PC if cond met and jump if cond
        let jump_to_a = or(jle, jgt);
        // And(a=Cinstruction, b=jumpToA, out=PCload); // Only jump if C instruction
        let pc_load = and(c_instruction, jump_to_a);
        // Not(in=PCload, out=PCinc);                  // only inc if not load
        let pc_inc = not(pc_load);
        // PC(in=Aout, inc=PCinc, load=PCload, reset=reset, out[0..14]=pc);
        self.pc.tick(a_out, reset, pc_load, pc_inc);
        let out = self.pc.tock();
        let pc = [
            out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7], out[8], out[9],
            out[10], out[11], out[12], out[13], out[14],
        ];

        (out_m, write_m, address_m, pc)
    }
}

fn flat(x: [bool; 16]) -> usize {
    x.iter()
        .enumerate()
        .fold(0, |acc, (i, &b)| if b { acc + (1 << i) } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tools;

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
            let data = convert16(t[9].parse::<i16>().unwrap());

            (
                is_set,
                input,
                instruction,
                reset,
                out,
                write_m,
                bit_16_to_15(address),
                bit_16_to_15(pc),
                data,
            )
        });

        let mut cpu = CPU::new();

        while let Some((is_set, input, instruction, reset, _, _, _, _, data)) = iter.next() {
            assert!(is_set);
            let (out, write_m, address, pc) = cpu.calc(input, instruction, reset);
            let (_, _, _, _, t_out, t_write_m, t_address, t_pc, _) = iter.next().unwrap();
            if let Some(t_out) = t_out {
                assert_eq!(out, t_out, "{} {}", flat(out), flat(t_out));
            }
            assert_eq!(write_m, t_write_m);
            assert_eq!(address, t_address);
            assert_eq!(pc, t_pc);
        }
    }
}
