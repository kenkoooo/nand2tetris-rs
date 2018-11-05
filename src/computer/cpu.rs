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
        let a_instruction_p = not(instruction[15]);
        let write_m = and(instruction[15], instruction[3]);

        let new_a = gates::mux16(self.alu_out, instruction, a_instruction_p);
        let store_ap1 = and(instruction[5], instruction[15]);
        let store_ap = or(store_ap1, a_instruction_p);

        self.address_register.tick(new_a, store_ap);
        let stored_a = self.address_register.tock();

        let address_m = [
            stored_a[0],
            stored_a[1],
            stored_a[2],
            stored_a[3],
            stored_a[4],
            stored_a[5],
            stored_a[6],
            stored_a[7],
            stored_a[8],
            stored_a[9],
            stored_a[10],
            stored_a[11],
            stored_a[12],
            stored_a[13],
            stored_a[14],
        ];

        let alu_in_am = gates::mux16(stored_a, input, instruction[12]);

        let (out_m, zr, ng) = alu::alu(
            self.data_register.tock(),
            alu_in_am,
            instruction[11],
            instruction[10],
            instruction[9],
            instruction[8],
            instruction[7],
            instruction[6],
        );

        println!("d={}", flat(self.data_register.tock()));
        println!("a/m={}", flat(alu_in_am));
        println!("{:?}", &instruction[6..12]);
        println!("out={}", flat(out_m));

        self.alu_out = out_m;
        let store_dp = and(instruction[4], instruction[15]);

        self.data_register.tick(self.alu_out, store_dp);

        let zr_inv = not(zr);
        let ng_inv = not(ng);

        let jgt1 = and(zr_inv, ng_inv);
        let jgt = and(instruction[0], jgt1);

        let jeq = and(zr, instruction[1]);
        let load1 = or(jeq, jgt);

        let jlt = and(ng, instruction[2]);
        let load2 = or(jlt, load1);
        let load3 = and(load2, instruction[15]);

        self.pc.tick(stored_a, reset, load3, true);
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
            assert_eq!(
                cpu.data_register.tock(),
                data,
                "{} {}",
                flat(cpu.data_register.tock()),
                flat(data)
            );
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
