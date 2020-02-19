use chip8;

fn main() -> Result<(), chip8::Error> {
    let mut emu = chip8::Emulator::new().expect("Failed to create emulator.");
    emu.run()
}
