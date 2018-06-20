use std::fmt;

use memory;

pub struct CPU {
    memory: memory::Memory,
    pc: u16,
    a: i8,
    b: i8,
    c: i8,
    d: i8,
    e: i8,
    h: i8,
    l: i8,
    fz: bool,
    fc: bool,
}

enum Flags {
    Z,
    C,
    Always,
}

impl Flags {
    fn get(&self, cpu: &CPU) -> bool {
        match self {
            Flags::Z => cpu.fz,
            Flags::C => cpu.fc,
            Flags::Always => true,
        }
    }

    fn set(&self, cpu: &mut CPU, value: bool) {
        match self {
            Flags::Z => {
                cpu.fz = value;
            }
            Flags::C => {
                cpu.fc = value;
            }
            _ => panic!("Unsetable!"),
        };
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            Flags::Z => "Z",
            Flags::C => "C",
            _ => "",
        };

        write!(f, "{}", result)
    }
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
    DE,
}

impl Register {
    fn read(&self, cpu: &CPU) -> i16 {
        match self {
            Register::A => cpu.a as i16,
            Register::B => cpu.b as i16,
            Register::C => cpu.c as i16,
            Register::D => cpu.d as i16,
            Register::E => cpu.e as i16,
            Register::H => cpu.h as i16,
            Register::L => cpu.l as i16,
            Register::BC => ((cpu.b as u16) << 8 | (cpu.c as u8) as u16) as i16,
            Register::HL => ((cpu.h as u16) << 8 | (cpu.l as u8) as u16) as i16,
            Register::DE => ((cpu.d as u16) << 8 | (cpu.e as u8) as u16) as i16,
        }
    }

    fn write(&self, cpu: &mut CPU, value: i16) {
        match self {
            Register::A => cpu.a = value as i8,
            Register::B => cpu.b = value as i8,
            Register::C => cpu.c = value as i8,
            Register::D => cpu.d = value as i8,
            Register::E => cpu.e = value as i8,
            Register::H => cpu.h = value as i8,
            Register::L => cpu.l = value as i8,
            Register::BC => {
                cpu.b = (value >> 8) as i8;
                cpu.c = value as i8;
            }
            Register::HL => {
                cpu.h = (value >> 8) as i8;
                cpu.l = value as i8;
            }
            Register::DE => {
                cpu.d = (value >> 8) as i8;
                cpu.e = value as i8;
            }
        };
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            Register::A => "A",
            Register::B => "B",
            Register::C => "C",
            Register::D => "D",
            Register::E => "E",
            Register::H => "H",
            Register::L => "L",
            Register::BC => "BC",
            Register::HL => "HL",
            Register::DE => "DE",
        };

        write!(f, "{}", result)
    }
}

enum Operand {
    AddressU8(u16),
    AddressU16(u16),
    Move(u16), // u16 is the address of number to move by
    Flag(Flags),
    NotFlag(Flags),
    Register(Register),
    RegisterAddressU8(Register), // Decay to AddressU8
    RegisterAddressU16(Register), // Decay to AddressU16
}

impl Operand {
    fn get(&self, cpu: &CPU) -> i16 {
        match self {
            Operand::AddressU8(address) => {
                cpu.memory.get_byte(*address).unwrap() as i16
            }
            Operand::AddressU16(address) => {
                cpu.memory.get_word(*address).unwrap()
            }
            Operand::Register(register) => register.read(cpu),
            Operand::RegisterAddressU8(register) => {
                let result = register.read(cpu);
                Operand::AddressU8(result as u16).get(cpu)
            }
            Operand::RegisterAddressU16(register) => {
                let result = register.read(cpu);
                Operand::AddressU16(result as u16).get(cpu)
            }
            Operand::Flag(flag) => flag.get(cpu) as i16,
            Operand::NotFlag(flag) => !flag.get(cpu) as i16,
            Operand::Move(address) => {
                let by = cpu.memory.get_byte(*address).unwrap();
                ((cpu.pc as i32) + by as i32) as i16
            }
        }
    }

    fn set(&self, cpu: &mut CPU, value: i16) {
        match self {
            Operand::AddressU8(address) => {
                cpu.memory.set_byte(*address, value as i8);
            }
            Operand::AddressU16(address) => {
                cpu.memory.set_word(*address, value);
            }
            Operand::Register(register) => {
                register.write(cpu, value);
            }
            Operand::RegisterAddressU8(register) => {
                let address = register.read(cpu) as u16;
                Operand::AddressU8(address).set(cpu, value);
            }
            Operand::RegisterAddressU16(register) => {
                let address = register.read(cpu);
                Operand::AddressU16(address as u16).set(cpu, value);
            }
            Operand::Flag(flag) => flag.set(cpu, value != 0),
            Operand::NotFlag(flag) => flag.set(cpu, value != 0),
            Operand::Move(_) => panic!("Can not set move"),
        };
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::AddressU8(address) => write!(f, "({:04X})", address),
            Operand::AddressU16(address) => write!(f, "({:04X})", address),
            Operand::Register(register) => register.fmt(f),
            Operand::RegisterAddressU8(register) => write!(f, "({})", register),
            Operand::RegisterAddressU16(register) => {
                write!(f, "({})", register)
            }
            Operand::Flag(flag) => flag.fmt(f),
            Operand::NotFlag(flag) => write!(f, "!{}", flag),
            Operand::Move(address) => write!(f, "PC+({:04X})", address),
        }
    }
}

enum Instructions {
    Undefined { opcode: u8 },
    Nop,
    Add { op1: Operand, op2: Operand },
    Xor { op1: Operand, op2: Operand },
    Load { op1: Operand, op2: Operand },
    Jp { cod: Operand, op: Operand },
    Inc { op: Operand },
    Dec { op: Operand },
    Sla { op: Operand },
    Stacked { stack: Vec<Instructions> },
    Di,
    Cpl,
}

impl Instructions {
    fn execute(&self, cpu: &mut CPU) {
        let result: i16;
        match self {
            Instructions::Undefined { opcode } => panic!(
                "{:02X}: Not identified on Address 0x{:04X}",
                opcode, cpu.pc
            ),
            Instructions::Nop => {}
            Instructions::Add { op1, op2 } => {
                result = op2.get(cpu);
                let op1_value = op1.get(cpu) + result;
                op1.set(cpu, op1_value);
            }
            Instructions::Xor { op1, op2 } => {
                let op1_value = op1.get(cpu);
                let result = op1_value ^ op2.get(cpu);
                op1.set(cpu, result);
            }
            Instructions::Jp { cod, op } => {
                if cod.get(cpu) > 0 {
                    cpu.pc = op.get(cpu) as u16;
                }
            }
            Instructions::Load { op1, op2 } => {
                result = op2.get(cpu);
                op1.set(cpu, result);
            }
            Instructions::Inc { op } => {
                result = op.get(cpu) + 1;
                op.set(cpu, result);
            }
            Instructions::Dec { op } => {
                result = op.get(cpu) - 1;
                op.set(cpu, result);

                cpu.fz = result == 0;
            }
            Instructions::Sla { op } => {
                result = op.get(cpu) << 1;
                op.set(cpu, result);
            }
            Instructions::Stacked { stack } => {
                for instr in stack.iter() {
                    instr.execute(cpu);
                }
            }
            Instructions::Cpl => {
                result = !Operand::Register(Register::A).get(cpu);
                Operand::Register(Register::A).set(cpu, result);
            }
            Instructions::Di => {}
        };
    }

    fn decode(opcode: u8, pc: &u16) -> (Instructions, u8) {
        let mut result = 1;
        let instr = match opcode {
            0x00 => Instructions::Nop,
            0x01 => {
                result += 2;

                Instructions::Load {
                    op1: Operand::Register(Register::BC),
                    op2: Operand::AddressU16(pc + 1),
                }
            }
            0x03 => Instructions::Inc {
                op: Operand::Register(Register::BC),
            },
            0x05 => Instructions::Dec {
                op: Operand::Register(Register::B),
            },
            0x06 => {
                result += 1;

                Instructions::Load {
                    op1: Operand::Register(Register::B),
                    op2: Operand::AddressU8(pc + 1),
                }
            }
            0x0B => Instructions::Dec {
                op: Operand::Register(Register::BC),
            },
            0x0D => Instructions::Dec {
                op: Operand::Register(Register::C),
            },
            0x0E => {
                result += 1;

                Instructions::Load {
                    op1: Operand::Register(Register::C),
                    op2: Operand::AddressU8(pc + 1),
                }
            }
            0x11 => {
                result += 2;

                Instructions::Load {
                    op1: Operand::Register(Register::DE),
                    op2: Operand::AddressU16(pc + 1),
                }
            }
            0x15 => Instructions::Dec {
                op: Operand::Register(Register::D),
            },
            0x19 => Instructions::Add {
                op1: Operand::Register(Register::HL),
                op2: Operand::Register(Register::DE),
            },
            0x20 => {
                result += 1;

                Instructions::Jp {
                    cod: Operand::NotFlag(Flags::Z),
                    op: Operand::Move(pc + 1),
                }
            }
            0x1D => Instructions::Dec {
                op: Operand::Register(Register::E),
            },
            0x21 => {
                result += 2;

                Instructions::Load {
                    op1: Operand::Register(Register::HL),
                    op2: Operand::AddressU16(pc + 1),
                }
            }
            0x22 => Instructions::Sla {
                op: Operand::Register(Register::D),
            },
            0x25 => Instructions::Dec {
                op: Operand::Register(Register::H),
            },
            0x29 => Instructions::Add {
                op1: Operand::Register(Register::HL),
                op2: Operand::Register(Register::HL),
            },
            0x32 => {
                let mut stacked_instructions = Vec::new();

                stacked_instructions.push(Instructions::Load {
                    op1: Operand::RegisterAddressU8(Register::HL),
                    op2: Operand::Register(Register::A),
                });

                stacked_instructions.push(Instructions::Dec {
                    op: Operand::Register(Register::HL),
                });

                Instructions::Stacked {
                    stack: stacked_instructions,
                }
            }
            0x3E => {
                result += 1;

                Instructions::Load {
                    op1: Operand::Register(Register::A),
                    op2: Operand::AddressU8(pc + 1),
                }
            }
            0x51 => Instructions::Load {
                op1: Operand::Register(Register::D),
                op2: Operand::Register(Register::C),
            },
            0x52 => Instructions::Load {
                op1: Operand::Register(Register::D),
                op2: Operand::Register(Register::D),
            },
            0x53 => Instructions::Load {
                op1: Operand::Register(Register::D),
                op2: Operand::Register(Register::E),
            },
            0x54 => Instructions::Load {
                op1: Operand::Register(Register::D),
                op2: Operand::Register(Register::H),
            },
            0x57 => Instructions::Load {
                op1: Operand::Register(Register::D),
                op2: Operand::Register(Register::A),
            },
            0x4B => Instructions::Load {
                op1: Operand::Register(Register::C),
                op2: Operand::Register(Register::E),
            },
            0x4A => Instructions::Load {
                op1: Operand::Register(Register::C),
                op2: Operand::Register(Register::D),
            },
            0x48 => Instructions::Load {
                op1: Operand::Register(Register::C),
                op2: Operand::Register(Register::B),
            },
            0x49 => Instructions::Load {
                op1: Operand::Register(Register::C),
                op2: Operand::Register(Register::C),
            },
            0x50 => Instructions::Load {
                op1: Operand::Register(Register::D),
                op2: Operand::Register(Register::B),
            },
            0x55 => Instructions::Load {
                op1: Operand::Register(Register::D),
                op2: Operand::Register(Register::L),
            },
            0x6C => Instructions::Load {
                op1: Operand::Register(Register::L),
                op2: Operand::Register(Register::H),
            },
            0x58 => Instructions::Load {
                op1: Operand::Register(Register::E),
                op2: Operand::Register(Register::B),
            },
            0x59 => Instructions::Load {
                op1: Operand::Register(Register::E),
                op2: Operand::Register(Register::C),
            },
            0x56 => Instructions::Load {
                op1: Operand::Register(Register::D),
                op2: Operand::RegisterAddressU8(Register::HL),
            },
            0x66 => Instructions::Load {
                op1: Operand::Register(Register::H),
                op2: Operand::RegisterAddressU8(Register::HL),
            },
            0x6E => Instructions::Load {
                op1: Operand::Register(Register::L),
                op2: Operand::RegisterAddressU8(Register::HL),
            },
            0x2C => Instructions::Inc {
                op: Operand::Register(Register::L),
            },
            0x2F => Instructions::Cpl,
            0xAF => Instructions::Xor {
                op1: Operand::Register(Register::A),
                op2: Operand::Register(Register::A),
            },
            0xC3 => {
                result += 2;

                Instructions::Jp {
                    cod: Operand::Flag(Flags::Always),
                    op: Operand::AddressU16(pc + 1),
                }
            }
            0xF3 => Instructions::Di,
            _ => Instructions::Undefined { opcode },
        };
        (instr, result)
    }
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instructions::Undefined { opcode } => {
                write!(f, "Undefined {:04X}", opcode)
            }
            Instructions::Nop => write!(f, "Nop"),
            Instructions::Add { op1, op2 } => write!(f, "Add {}, {}", op1, op2),
            Instructions::Xor { op1, op2 } => write!(f, "Xor {}, {}", op1, op2),
            Instructions::Jp { cod, op } => write!(f, "Jp {}, {}", cod, op),
            Instructions::Load { op1, op2 } => {
                write!(f, "Load {}, {}", op1, op2)
            }
            Instructions::Inc { op } => write!(f, "Inc {}", op),
            Instructions::Dec { op } => write!(f, "Dec {}", op),
            Instructions::Sla { op } => write!(f, "Sla {}", op),
            Instructions::Stacked { stack } => {
                for instr in stack.iter() {
                    let _ = write!(f, "{} -- ", instr);
                }

                write!(f, "")
            }
            Instructions::Cpl => write!(f, "Cpl"),
            Instructions::Di => write!(f, "Di -- Not functional"),
        }
    }
}

impl CPU {
    pub fn step(&mut self) {
        let opcode = self.memory.get_byte(self.pc).unwrap();
        let (instruction, size) =
            Instructions::decode(opcode as u8, &mut self.pc);
        println!("0x{:04X} -- {}: {}", self.pc, size, instruction);
        self.pc += size as u16;
        instruction.execute(self);
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
            fz: false,
            fc: false,
        }
    }
}
