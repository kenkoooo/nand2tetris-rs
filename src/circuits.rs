use circuits;
use gates;

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
        let input = gates::mux(self.dff.after, input, load);
        eprintln!("input={}", input);
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
        let mut result = [false; 16];
        result[0] = self.bits[0].tock();
        result[1] = self.bits[1].tock();
        result[2] = self.bits[2].tock();
        result[3] = self.bits[3].tock();
        result[4] = self.bits[4].tock();
        result[5] = self.bits[5].tock();
        result[6] = self.bits[6].tock();
        result[7] = self.bits[7].tock();
        result[8] = self.bits[8].tock();
        result[9] = self.bits[9].tock();
        result[10] = self.bits[10].tock();
        result[11] = self.bits[11].tock();
        result[12] = self.bits[12].tock();
        result[13] = self.bits[13].tock();
        result[14] = self.bits[14].tock();
        result[15] = self.bits[15].tock();
        result
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
            eprintln!("{:?}", t);
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

    fn convert16(x: i16) -> [bool; 16] {
        let mut result = [false; 16];
        for i in 0..15 {
            result[i] = (1 << i) & x != 0;
        }
        result[15] = x < 0;
        result
    }
}
