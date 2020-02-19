#[derive(Debug)]
pub enum Error {
    AddressOutOfBounds,
    InvalidOpcode,
}

pub const MEMORY_SIZE: usize = 4096;

pub struct Emulator {
    memory: [u8; MEMORY_SIZE],
    data_registers: [u8; 16],
    address_register: u16,
    stack_pointer: u16,
    program_counter: u16,
}

impl Emulator {
    pub fn new() -> Result<Emulator, Error> {
        Ok(Emulator {
            memory: [0; MEMORY_SIZE],
            data_registers: [0; 16],
            address_register: 0,
            stack_pointer: 0,
            program_counter: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn read16(&mut self, addr: impl Into<usize>) -> Result<u16, Error> {
        let addr = addr.into();

        if addr <= MEMORY_SIZE - 2 {
            Ok(((self.memory[addr] as u16) << 8) | self.memory[addr + 1] as u16)
        } else {
            Err(Error::AddressOutOfBounds)
        }
    }

    fn write16(&mut self, addr: impl Into<usize>, value: u16) {
        let addr = addr.into();
        self.memory[addr] = (value / 0x100) as u8;
        self.memory[addr + 1] = (value % 0x100) as u8;
    }

    fn push(&mut self, value: u16) {
        self.stack_pointer += 2;
        self.write16(self.stack_pointer, value);
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let opcode = self.read16(self.program_counter as usize)?;

        match (
            opcode / 0x1000,
            (opcode / 0x100) % 0x10,
            (opcode / 0x10) % 0x10,
            opcode % 0x10,
        ) {
            (0, 0, 0xE, 0) => todo!("Clear the screen."),
            (0, 0, 0xE, 0xE) => todo!("Return."),
            (1, _, _, _) => {
                self.program_counter = (opcode % 0x1000) as u16;
                Ok(())
            }
            (2, _, _, _) => {
                self.push(self.program_counter + 2);
                self.program_counter = (opcode % 0x1000) as u16;
                Ok(())
            }
            (3, x, _, _) => {
                if self.data_registers[x as usize] == (opcode % 0x100) as u8 {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
                Ok(())
            }
            (4, x, _, _) => {
                if self.data_registers[x as usize] != (opcode % 0x100) as u8 {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
                Ok(())
            }
            (5, x, y, 0) => {
                if self.data_registers[x as usize] == self.data_registers[y as usize] {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
                Ok(())
            }
            (6, x, _, _) => {
                self.data_registers[x as usize] = (opcode % 0x100) as u8;
                self.program_counter += 2;
                Ok(())
            }
            (7, x, _, _) => {
                self.data_registers[x as usize] += (opcode % 0x100) as u8;
                Ok(())
            }
            (8, x, y, 0) => {
                self.data_registers[x as usize] = self.data_registers[y as usize];
                Ok(())
            }
            (8, x, y, 1) => {
                self.data_registers[x as usize] |= self.data_registers[y as usize];
                Ok(())
            }
            (8, x, y, 2) => {
                self.data_registers[x as usize] &= self.data_registers[y as usize];
                Ok(())
            }
            (8, x, y, 3) => {
                self.data_registers[x as usize] ^= self.data_registers[y as usize];
                Ok(())
            }
            (8, x, y, 4) => todo!("Register addition with carry flag."),
            (8, x, y, 5) => todo!("Registar subtraction with borrow flag."),
            (8, x, _, 6) => {
                self.data_registers[0xF] = self.data_registers[x as usize] & 1;
                self.data_registers[x as usize] >>= 1;
                Ok(())
            }
            (8, x, y, 7) => todo!("Flipped register subtraction with borrow flag."),
            (8, x, _, 0xE) => {
                self.data_registers[0xF] = self.data_registers[x as usize] & 0b1000_0000;
                self.data_registers[x as usize] <<= 1;
                Ok(())
            }
            (9, x, y, 0) => {
                if self.data_registers[x as usize] != self.data_registers[y as usize] {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
                Ok(())
            }
            (0xA, _, _, _) => {
                self.address_register = opcode % 0x1000;
                Ok(())
            }
            (0xB, _, _, _) => {
                self.program_counter = self.data_registers[0] as u16 + (opcode % 0x1000);
                Ok(())
            }
            (0xC, x, _, _) => todo!("Random number ANDed with NN."),
            (0xD, x, y, n) => todo!("Draw sprite."),
            (0xE, x, 9, 0xE) => todo!("Skip if key pressed."),
            (0xE, x, 0xA, 1) => todo!("Skip if key not pressed."),
            (0xF, x, 0, 7) => todo!("Store value of delay timer in register."),
            (0xF, x, 0, 0xA) => todo!("Wait on key press."),
            _ => Err(Error::InvalidOpcode),
        }
    }
}
