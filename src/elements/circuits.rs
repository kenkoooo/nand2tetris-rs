use elements::alu;
use elements::gates::*;

struct DFlipFlop {
    before: bool,
    after: bool,
}

impl DFlipFlop {
    fn new() -> Self {
        DFlipFlop {
            before: false,
            after: false,
        }
    }
    fn tick(&mut self, input: bool) {
        self.before = input;
    }
    fn tock(&mut self) -> bool {
        self.after = self.before;
        self.after
    }
}

pub struct Bit {
    dff: DFlipFlop,
}

impl Bit {
    pub fn new() -> Self {
        Bit {
            dff: DFlipFlop::new(),
        }
    }
    pub fn tick(&mut self, input: bool, load: bool) {
        let input = mux(self.dff.after, input, load);
        self.dff.tick(input);
    }

    pub fn tock(&mut self) -> bool {
        self.dff.tock()
    }
}

pub struct Register {
    bits: [Bit; 16],
}

impl Register {
    pub fn new() -> Self {
        Register {
            bits: [
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
            ],
        }
    }
    pub fn tick(&mut self, input: [bool; 16], load: bool) {
        self.bits[0].tick(input[0], load);
        self.bits[1].tick(input[1], load);
        self.bits[2].tick(input[2], load);
        self.bits[3].tick(input[3], load);
        self.bits[4].tick(input[4], load);
        self.bits[5].tick(input[5], load);
        self.bits[6].tick(input[6], load);
        self.bits[7].tick(input[7], load);
        self.bits[8].tick(input[8], load);
        self.bits[9].tick(input[9], load);
        self.bits[10].tick(input[10], load);
        self.bits[11].tick(input[11], load);
        self.bits[12].tick(input[12], load);
        self.bits[13].tick(input[13], load);
        self.bits[14].tick(input[14], load);
        self.bits[15].tick(input[15], load);
    }
    pub fn tock(&mut self) -> [bool; 16] {
        [
            self.bits[0].tock(),
            self.bits[1].tock(),
            self.bits[2].tock(),
            self.bits[3].tock(),
            self.bits[4].tock(),
            self.bits[5].tock(),
            self.bits[6].tock(),
            self.bits[7].tock(),
            self.bits[8].tock(),
            self.bits[9].tock(),
            self.bits[10].tock(),
            self.bits[11].tock(),
            self.bits[12].tock(),
            self.bits[13].tock(),
            self.bits[14].tock(),
            self.bits[15].tock(),
        ]
    }
}

pub fn load_address3(address: [bool; 3]) -> [bool; 8] {
    [
        and(not(address[0]), and(not(address[1]), not(address[2]))),
        and(address[0], and(not(address[1]), not(address[2]))),
        and(not(address[0]), and(address[1], not(address[2]))),
        and(address[0], and(address[1], not(address[2]))),
        and(not(address[0]), and(not(address[1]), address[2])),
        and(address[0], and(not(address[1]), address[2])),
        and(not(address[0]), and(address[1], address[2])),
        and(address[0], and(address[1], address[2])),
    ]
}

pub fn and16way(input: [bool; 16], sel: bool) -> [bool; 16] {
    [
        and(input[0], sel),
        and(input[1], sel),
        and(input[2], sel),
        and(input[3], sel),
        and(input[4], sel),
        and(input[5], sel),
        and(input[6], sel),
        and(input[7], sel),
        and(input[8], sel),
        and(input[9], sel),
        and(input[10], sel),
        and(input[11], sel),
        and(input[12], sel),
        and(input[13], sel),
        and(input[14], sel),
        and(input[15], sel),
    ]
}

pub struct RAM8 {
    registers: [Register; 8],
}

impl RAM8 {
    pub fn new() -> Self {
        RAM8 {
            registers: [
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
            ],
        }
    }

    pub fn set(&mut self, input: [bool; 16], addr: [bool; 3], load: bool) {
        let addr = load_address3(addr);
        self.registers[0].tick(input, and(addr[0], load));
        self.registers[1].tick(input, and(addr[1], load));
        self.registers[2].tick(input, and(addr[2], load));
        self.registers[3].tick(input, and(addr[3], load));
        self.registers[4].tick(input, and(addr[4], load));
        self.registers[5].tick(input, and(addr[5], load));
        self.registers[6].tick(input, and(addr[6], load));
        self.registers[7].tick(input, and(addr[7], load));
    }

    pub fn get(&mut self, address: [bool; 3]) -> [bool; 16] {
        let address = load_address3(address);
        let a = or16(
            and16way(self.registers[0].tock(), address[0]),
            and16way(self.registers[1].tock(), address[1]),
        );
        let a = or16(a, and16way(self.registers[2].tock(), address[2]));
        let a = or16(a, and16way(self.registers[3].tock(), address[3]));
        let a = or16(a, and16way(self.registers[4].tock(), address[4]));
        let a = or16(a, and16way(self.registers[5].tock(), address[5]));
        let a = or16(a, and16way(self.registers[6].tock(), address[6]));
        or16(a, and16way(self.registers[7].tock(), address[7]))
    }
}

pub struct ProgramCounter {
    register: Register,
    out: [bool; 16],
}

impl ProgramCounter {
    pub fn new() -> Self {
        ProgramCounter {
            register: Register::new(),
            out: [false; 16],
        }
    }

    pub fn tick(&mut self, input: [bool; 16], reset: bool, load: bool, inc: bool) {
        let input = and16way(input, and(load, not(reset)));
        let increased = and16way(alu::inc16(self.out), and(inc, and(not(load), not(reset))));
        let input = or16(input, increased);
        let out = and16way(self.out, and(not(reset), and(not(load), not(inc))));
        self.register.tick(or16(input, out), true);
    }

    pub fn tock(&mut self) -> [bool; 16] {
        self.out = self.register.tock();
        self.out
    }
}

pub struct RAM64 {
    ram8s: [RAM8; 8],
}

impl RAM64 {
    pub fn new() -> Self {
        RAM64 {
            ram8s: [
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
            ],
        }
    }

    pub fn set(&mut self, input: [bool; 16], addr: [bool; 6], load: bool) {
        let addr3 = load_address3([addr[0], addr[1], addr[2]]);
        let addr6 = [addr[3], addr[4], addr[5]];
        self.ram8s[0].set(input, addr6, and(addr3[0], load));
        self.ram8s[1].set(input, addr6, and(addr3[1], load));
        self.ram8s[2].set(input, addr6, and(addr3[2], load));
        self.ram8s[3].set(input, addr6, and(addr3[3], load));
        self.ram8s[4].set(input, addr6, and(addr3[4], load));
        self.ram8s[5].set(input, addr6, and(addr3[5], load));
        self.ram8s[6].set(input, addr6, and(addr3[6], load));
        self.ram8s[7].set(input, addr6, and(addr3[7], load));
    }

    pub fn get(&mut self, addr: [bool; 6]) -> [bool; 16] {
        let addr3 = load_address3([addr[0], addr[1], addr[2]]);
        let addr6 = [addr[3], addr[4], addr[5]];
        let a = or16(
            and16way(self.ram8s[0].get(addr6), addr3[0]),
            and16way(self.ram8s[1].get(addr6), addr3[1]),
        );
        let a = or16(a, and16way(self.ram8s[2].get(addr6), addr3[2]));
        let a = or16(a, and16way(self.ram8s[3].get(addr6), addr3[3]));
        let a = or16(a, and16way(self.ram8s[4].get(addr6), addr3[4]));
        let a = or16(a, and16way(self.ram8s[5].get(addr6), addr3[5]));
        let a = or16(a, and16way(self.ram8s[6].get(addr6), addr3[6]));
        or16(a, and16way(self.ram8s[7].get(addr6), addr3[7]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tools;
    #[test]
    fn bit_test() {
        let t = tools::read_test_data("tests/03/Bit.cmp").unwrap();

        let mut iter = t[1..].iter().map(|t| {
            let input = t[2].parse::<usize>().unwrap() == 1;
            let load = t[3].parse::<usize>().unwrap() == 1;
            let out = t[4].parse::<usize>().unwrap() == 1;
            (input, load, out)
        });
        let mut bit = Bit {
            dff: DFlipFlop {
                before: false,
                after: false,
            },
        };
        while let Some((input, load, _)) = iter.next() {
            bit.tick(input, load);
            let (_, _, out) = iter.next().unwrap();
            assert_eq!(bit.tock(), out);
        }
    }
    #[test]
    fn register_test() {
        let t = tools::read_test_data("tests/03/Register.cmp").unwrap();
        let mut iter = t[1..].iter().map(|t| {
            let input = convert16(t[2].parse::<i16>().unwrap());
            let load = t[3].parse::<i16>().unwrap() == 1;
            let output = convert16(t[4].parse::<i16>().unwrap());
            (input, load, output)
        });

        let mut register = Register::new();

        while let Some((input, load, _)) = iter.next() {
            register.tick(input, load);
            let (_, _, out) = iter.next().unwrap();
            assert_eq!(register.tock(), out);
        }
    }

    #[test]
    fn ram8_test() {
        let t = tools::read_test_data("tests/03/RAM8.cmp").unwrap();
        let mut iter = t[1..].iter().map(|t| {
            let is_set = t[1].chars().next_back().unwrap() == '+';
            let input = convert16(t[2].parse::<i16>().unwrap());
            let load = t[3].parse::<i16>().unwrap() == 1;
            let address = convert3(t[4].parse::<i16>().unwrap());
            let output = convert16(t[5].parse::<i16>().unwrap());
            (is_set, input, load, address, output)
        });

        let mut ram8 = RAM8::new();

        while let Some((is_set, input, load, address, out)) = iter.next() {
            if is_set {
                ram8.set(input, address, load);
            } else {
                assert_eq!(ram8.get(address), out);
            }
        }
    }

    #[test]
    fn pc_test() {
        let t = tools::read_test_data("tests/03/PC.cmp").unwrap();
        let mut iter = t[1..].iter().map(|t| {
            let is_set = t[1].chars().next_back().unwrap() == '+';
            let input = convert16(t[2].parse::<i16>().unwrap());
            let reset = t[3].parse::<i16>().unwrap() == 1;
            let load = t[4].parse::<i16>().unwrap() == 1;
            let inc = t[5].parse::<i16>().unwrap() == 1;
            let output = convert16(t[6].parse::<i16>().unwrap());
            (is_set, input, reset, load, inc, output)
        });

        let mut pc = ProgramCounter::new();

        while let Some((is_set, input, reset, load, inc, output)) = iter.next() {
            if is_set {
                pc.tick(input, reset, load, inc);
            } else {
                assert_eq!(pc.tock(), output);
            }
        }
    }

    #[test]
    fn ram64_test() {
        let t = tools::read_test_data("tests/03/RAM64.cmp").unwrap();
        let mut iter = t[1..].iter().map(|t| {
            let is_set = t[1].chars().next_back().unwrap() == '+';
            let input = convert16(t[2].parse::<i16>().unwrap());
            let load = t[3].parse::<i16>().unwrap() == 1;
            let address = convert6(t[4].parse::<i16>().unwrap());
            let output = convert16(t[5].parse::<i16>().unwrap());
            (is_set, input, load, address, output)
        });

        let mut ram64 = RAM64::new();

        while let Some((is_set, input, load, address, out)) = iter.next() {
            if is_set {
                ram64.set(input, address, load);
            } else {
                assert_eq!(ram64.get(address), out);
            }
        }
    }

    fn convert3(x: i16) -> [bool; 3] {
        [x & 1 != 0, x & 2 != 0, x & 4 != 0]
    }

    fn convert6(x: i16) -> [bool; 6] {
        [
            x & 1 != 0,
            x & 2 != 0,
            x & 4 != 0,
            x & 8 != 0,
            x & 16 != 0,
            x & 32 != 0,
        ]
    }

    fn convert16(x: i16) -> [bool; 16] {
        let mut result = [false; 16];
        for i in 0..15 {
            result[i] = (1 << i) & x != 0;
        }
        result[15] = x < 0;
        result
    }
}
