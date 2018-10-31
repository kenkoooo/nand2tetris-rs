use circuits;
use gates;

struct DFlipFlop {
    out: bool,
}

impl DFlipFlop {
    fn bit(&mut self, i: bool, load: bool) -> bool {
        let out = gates::mux(i, self.out, load);
        self.out = out;
        out
    }
}
