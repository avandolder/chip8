pub struct Emulator;

#[derive(Debug)]
pub enum Error {}

impl Emulator {
    pub fn new() -> Result<Emulator, Error> {
        Ok(Emulator {})
    }

    pub fn run(&mut self) -> Result<(), Error> {
        Ok(())
    }
}