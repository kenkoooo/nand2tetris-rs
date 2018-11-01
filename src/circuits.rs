use circuits;
use gates;

struct DFlipFlop {
    before: bool,
    after: bool,
}

impl DFlipFlop {
    fn tick(&mut self, input: bool) {
        self.before = input;
    }
    fn tock(&mut self) -> bool {
        self.after = self.before;
        self.after
    }
}

struct Bit {
    dff: DFlipFlop,
}

impl Bit {
    fn tick(&mut self, input: bool, load: bool) {
        let input = gates::mux(self.dff.after, input, load);
        eprintln!("input={}", input);
        self.dff.tick(input);
    }
    fn tock(&mut self) -> bool {
        self.dff.tock()
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
}
