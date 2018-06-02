use memory;

pub struct CPU {
    memory: memory::Memory,
    pc: usize,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum Instructions {
    Undefined,
    Nop,
    Jp { nn: u16 },
    Load { r1: Register, r2: Register },
    Inc { r1: Register },
}

impl Instructions {
    fn decode(opcode: u8, cpu: &mut CPU) -> Instructions {
        cpu.pc += 1;
        match opcode {
            0x00 => Instructions::Nop,
            0xC3 => {
                let op = cpu.memory.get_word(cpu.pc).unwrap();
                cpu.pc = cpu.pc + 2;

                Instructions::Jp { nn: op }
            }
            0x54 => Instructions::Load {
                r1: Register::D,
                r2: Register::H,
            },
            0x4B => Instructions::Load {
                r1: Register::C,
                r2: Register::E,
            },
            0x50 => Instructions::Load {
                r1: Register::D,
                r2: Register::B,
            },
            0x2C => Instructions::Inc { r1: Register::L },
            _ => Instructions::Undefined,
        }
    }
}

impl CPU {
    fn get_register(&self, register: &Register) -> u16 {
        match register {
            Register::A => self.a as u16,
            Register::B => self.b as u16,
            Register::C => self.c as u16,
            Register::D => self.d as u16,
            Register::E => self.e as u16,
            Register::H => self.h as u16,
            Register::L => self.l as u16,
        }
    }

    fn set_register(&mut self, register: &Register, value: u16) {
        match register {
            Register::A => self.a = value as u8,
            Register::B => self.b = value as u8,
            Register::C => self.c = value as u8,
            Register::D => self.d = value as u8,
            Register::E => self.e = value as u8,
            Register::H => self.h = value as u8,
            Register::L => self.l = value as u8,
        };
    }

    pub fn step(&mut self) {
        let opcode = self.memory.get_byte(self.pc).unwrap();
        match Instructions::decode(opcode, self) {
            Instructions::Undefined => {
                panic!("{:02X}: Not identified on Address {:02X}", opcode, self.pc)
            }
            Instructions::Nop => self.pc = self.pc + 1,
            Instructions::Jp { nn } => self.pc = nn as usize,
            Instructions::Load { r1, r2 } => self.set_register(&r1, self.get_register(&r2)),
            Instructions::Inc { r1 } => self.set_register(&r1, self.get_register(&r1) + 1),
        };
    }

    pub fn new(memory: memory::Memory) -> CPU {
        CPU {
            memory,
            pc: 0x100,
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
        }
    }
}
