use memory;

pub struct CPU {
    memory : memory::Memory,
    pc : usize,
    a : u8,
    b : u8,
    c : u8,
    d : u8,
    e : u8,
    h : u8,
    l : u8,
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
    Jp{nn : u16},
    Load{r1 : Register, r2 : Register},
    Inc{r1 : Register},
}

impl Instructions {
    fn decode(opcode : u8, cpu : &mut CPU) -> Instructions {
        cpu.pc += 1;
        match opcode {
            0x00 => Instructions::Nop,
            0xC3 => {
                let op = cpu.memory.get_word(cpu.pc).unwrap();
                cpu.pc = cpu.pc + 2;

                Instructions::Jp{nn : op}
            },
            0x54 => Instructions::Load{r1 : Register::D, r2 : Register::H},
            0x4B => Instructions::Load{r1 : Register::C, r2 : Register::E},
            0x50 => Instructions::Load{r1 : Register::D, r2 : Register::B},
            0x2C => Instructions::Inc{r1 : Register::L},
            _ => Instructions::Undefined,
        }
    }
}

impl CPU {
    fn translate_register(&mut self, register : Register) -> &mut u8 {
        match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
            Register::E => &mut self.e,
            Register::H => &mut self.h,
            Register::L => &mut self.l,
        }
    }

    pub fn step(&mut self) {
        let opcode = self.memory.get_byte(self.pc).unwrap();
        match Instructions::decode(opcode, self) {
            Instructions::Undefined => panic!("{:02X}: Not identified on Address {:02X}",
                                                opcode, self.pc),
            Instructions::Nop => {
                self.pc = self.pc + 1;
            },
            Instructions::Jp{nn} => {
                self.pc = nn as usize;
            },
            Instructions::Load{r1, r2} => {
                let value_r2 : u8;
                {
                    let register_source = self.translate_register(r2);
                    value_r2 = *register_source;
                }
                let register_destiny = self.translate_register(r1);

                *register_destiny = value_r2;
            },
            Instructions::Inc{r1} => {
                let register = self.translate_register(r1);

                *register += 1;
            }
        };
    }

    pub fn new(memory : memory::Memory ) -> CPU {
        CPU {
            memory,
            pc : 0x100,
            a : 0x01,
            b : 0x00,
            c : 0x13,
            d : 0x00,
            e : 0xD8,
            h : 0x01,
            l : 0x4D,
        }
    }
}
