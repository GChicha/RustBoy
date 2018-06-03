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
    BC,
    HL,
}

trait Executable {
    fn execute();
}

enum Instructions {
    Undefined,
    Nop,
    Jp { nn: u16 },
    Load { r1: Register, r2: Register },
    LoadImmediate { r1 : Register, immediate : u16 },
    Inc { r1: Register },
    Dec { r1: Register },
}

impl Executable for Instructions {
    fn execute() {
    }
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
            0x4A => Instructions::Load {
                r1: Register::C,
                r2: Register::D,
            },
            0x48 => Instructions::Load {
                r1: Register::C,
                r2: Register::B,
            },
            0x50 => Instructions::Load {
                r1: Register::D,
                r2: Register::B,
            },
            0x55 => Instructions::Load {
                r1: Register::D,
                r2: Register::L,
            },
            0x6C => Instructions::Load {
                r1: Register::L,
                r2: Register::H,
            },
            0x58 => Instructions::Load {
                r1: Register::E,
                r2: Register::B,
            },
            0x59 => Instructions::Load {
                r1: Register::E,
                r2: Register::C,
            },
            0x01 => {
                let immediate = cpu.memory.get_word(cpu.pc).unwrap();
                cpu.pc = cpu.pc + 2;

                Instructions::LoadImmediate {
                    r1: Register::BC,
                    immediate,
                }
            },
            0x56 => {
                let immediate = cpu.memory.get_word(
                    cpu.get_register(&Register::HL) as usize).unwrap();
                cpu.pc = cpu.pc + 2;

                Instructions::LoadImmediate {
                    r1 : Register::D,
                    immediate,
                }
            },
            0x66 => {
                let immediate = cpu.memory.get_word(
                    cpu.get_register(&Register::HL) as usize).unwrap();
                cpu.pc = cpu.pc + 2;

                Instructions::LoadImmediate {
                    r1 : Register::H,
                    immediate,
                }
            },
            0x6E => {
                let immediate = cpu.memory.get_word(
                    cpu.get_register(&Register::HL) as usize).unwrap();
                cpu.pc = cpu.pc + 2;

                Instructions::LoadImmediate {
                    r1 : Register::L,
                    immediate,
                }
            },
            0x2C => Instructions::Inc { r1: Register::L },
            0x03 => Instructions::Inc { r1: Register::BC },
            0x0D => Instructions::Dec { r1: Register::C },
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
            Register::BC => {
                (self.get_register(&Register::B) << 8) |
                    self.get_register(&Register::C)
            },
            Register::HL => {
                (self.get_register(&Register::H) << 8) |
                    self.get_register(&Register::L)
            },
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
            Register::BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8;
            },
            Register::HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8;
            },
        };
    }

    pub fn step(&mut self) {
        let opcode = self.memory.get_byte(self.pc).unwrap();
        println!("On address {:04X} opcode {:02X}", self.pc, opcode);
        let result : u16;
        match Instructions::decode(opcode, self) {
            Instructions::Undefined => panic!(
                "{:02X}: Not identified on Address {:04X}", opcode, self.pc),
            Instructions::Nop => {},
            Instructions::Jp { nn } => {
                println!("Next pc = {:04X}", nn);
                self.pc = nn as usize;
            },
            Instructions::Load { r1, r2 } => {
                result = self.get_register(&r2);
                self.set_register(&r1, result);
            },
            Instructions::Inc { r1 } => {
                result = self.get_register(&r1) + 1;
                self.set_register(&r1, result);
            },
            Instructions::LoadImmediate { r1, immediate } => {
                self.set_register(&r1, immediate);
            },
            Instructions::Dec { r1 } => {
                result = self.get_register(&r1) - 1;
                self.set_register(&r1, result);
            },
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
