use computer::cpu::CPUTrait;
use computer::memory::MemoryTrait;
use emulator::rom::ROM32K;

pub struct Emulator<M, C> {
    memory: M,
    rom: ROM32K<M>,
    cpu: C,

    instruction: [bool; 16],
    in_m: [bool; 16],
    reset: bool,
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
