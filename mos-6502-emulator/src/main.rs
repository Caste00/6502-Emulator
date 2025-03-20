mod cpu;
mod memory;

use memory::memory::Memory;
use cpu::cpu::Cpu;

fn main() {
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();

    /* Debug */
    cpu.reset();
    cpu.x = 0xFF;

    mem.data[0xFFFC] = Cpu::LDA_ABSOLUTE_X;
    mem.data[0xFFFD] = 0x33;
    mem.data[0xFFFE] = 0x44;
    mem.data[0x4445] = 23;
    mem.data[0x4532] = 32;

    cpu.execute(5, &mut mem);

    println!("{:?}", cpu);

}
