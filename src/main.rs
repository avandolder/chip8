use chip8;

fn main() -> Result<(), chip8::Error> {
    let mut emu = chip8::Emulator::new().expect("Emulator failed to initialize.");
    emu.run()
}
