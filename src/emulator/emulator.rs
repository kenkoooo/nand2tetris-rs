use computer::cpu::CPUTrait;
use computer::memory::MemoryTrait;
use emulator::rom::ROM32K;

pub struct Emulator<M, C> {
    pub memory: M,
    pub rom: ROM32K<M>,
    pub cpu: C,

    pub instruction: [bool; 16],
    pub in_m: [bool; 16],
    pub reset: bool,
}

impl<M, C> Emulator<M, C>
where
    M: MemoryTrait,
    C: CPUTrait,
{
    pub fn new(prog: &Vec<[bool; 16]>) -> Self {
        Emulator {
            memory: M::new(),
            rom: ROM32K::new(prog),
            cpu: C::new(),
            instruction: [false; 16],
            in_m: [false; 16],
            reset: true,
        }
    }

    pub fn iterate(&mut self) {
        self.cpu.tick(self.in_m, self.instruction, self.reset);
        let (out_m, write_m, address_m, pc) = self.cpu.tock(self.in_m, self.instruction);
        self.in_m = self.memory.access(out_m, write_m, address_m);
        self.instruction = self.rom.access(pc);
        self.reset = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use computer::cpu::CPU;
    use emulator::memory::EmulatedMemory;
    use std::cmp;
    use std::thread;
    use tools;

    #[test]
    fn emulate_add_test() {
        let child = thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(move || {
                let t = tools::read_file("tests/06/add/Add.hack").unwrap();
                let binary = tools::load_hack_binary(&t);

                let mut emulator = Emulator::<EmulatedMemory, CPU>::new(&binary);

                for _ in 0..7 {
                    emulator.iterate();
                }
                let value = emulator
                    .memory
                    .access([false; 16], false, tools::convert_address(0));
                assert_eq!(value, tools::convert16(5));
            }).unwrap();
        child.join().unwrap();
    }

    #[test]
    fn emulate_max_test() {
        let child = thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(move || {
                let t = tools::read_file("tests/06/max/Max.hack").unwrap();
                let binary = tools::load_hack_binary(&t);

                for r0 in 0..100 {
                    for r1 in 0..100 {
                        let mut emulator = Emulator::<EmulatedMemory, CPU>::new(&binary);
                        emulator.memory.access(
                            tools::convert16(r0),
                            true,
                            tools::convert_address(0),
                        );
                        emulator.memory.access(
                            tools::convert16(r1),
                            true,
                            tools::convert_address(1),
                        );

                        for _ in 0..20 {
                            emulator.iterate();
                        }
                        let r2 =
                            emulator
                                .memory
                                .access([false; 16], false, tools::convert_address(2));
                        assert_eq!(r2, tools::convert16(cmp::max(r0, r1)));
                    }
                }
            }).unwrap();
        child.join().unwrap();
    }
}
