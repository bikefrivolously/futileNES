use super::cpu;

#[derive(Default)]
struct CpuState {
    cpu: cpu::CPU
}

impl CpuState {
    // will it work if CpuState has it's own CPU instance that it copies the real CPU state into?
    fn store(&mut self, cpu: &CPU) {
        self.cpu.reg_pc = cpu.reg_pc;
        self.cpu.reg_sp = cpu.reg_sp;
        self.cpu.reg_a = cpu.reg_a;
        self.cpu.reg_x = cpu.reg_x;
        self.cpu.reg_y = cpu.reg_y;
        self.cpu.reg_p = cpu.reg_p;
    }
}

pub struct Debugger {
    cpu: cpu::CPU,
    state: CpuState,
}

impl Debugger {
    pub fn new(cpu: cpu::CPU) -> Debugger {
        Debugger {
            cpu: cpu,
            state: CpuState::default(),
        }
    }
    pub fn step(&self) {
        state.store(&cpu);
        cpu.step();
        // TODO: figure out how to get the opcode and it's arguments from the CPU.
        // Currently their scope is limited and not accessible from outside the CPU's step method.
    }
}
