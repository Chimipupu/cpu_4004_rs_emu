use std::fmt;
use cpu_4004::i4004;
mod cpu_4004;

fn main() {
    let mut cpu = i4004::new();
    println!("Intel 4004 Emulator by Chimi (https://github.com/Chimipupu/cpu_4004_rs_emu)");

    // i4004のアセンブラ
    let program = [
        0x00,       // NOP
        0xFF,       // HALT
    ];

    cpu.load_program(&program);
    cpu.run();

    println!("{}", cpu);
}