pub struct Memory {
    rom: Vec<u8>,
}

enum Section {
    Rom,
}

struct TranslatedAddress {
    section: Section,
    address: u16,
}

impl Memory {
    fn translate_address(address: u16) -> Result<TranslatedAddress, &'static str> {
        if address <= 0 {
            Err("Memory must be higher than zero")
        } else if address < 0x8000 {
            Ok(TranslatedAddress {
                section: Section::Rom,
                address: address,
            })
        } else {
            Err("Not mapped yet")
        }
    }

    pub fn get_byte(&self, address: u16) -> Result<i8, &'static str> {
        let translate_address = Memory::translate_address(address)?;
        match translate_address.section {
            Section::Rom => Ok(self.rom[translate_address.address as usize] as i8),
            _ => Err("Not mapped yet"),
        }
    }

    pub fn get_word(&self, address: u16) -> Result<i16, &'static str> {
        let translate_address = Memory::translate_address(address)?;
        match translate_address.section {
            Section::Rom => {
                let l_data = self.get_byte(address).unwrap();
                let m_data = self.get_byte(address + 1).unwrap();

                let data = (((m_data as u16) << 8) | (l_data as u8) as u16) as i16;

                Ok(data)
            }
            _ => Err("Not mapped yet"),
        }
    }

    pub fn set_byte(&mut self, address: u16, value: i8) {
        let translate_address = Memory::translate_address(address).unwrap();
        match translate_address.section {
            Section::Rom => self.rom[translate_address.address as usize] = value as u8,
        };
    }

    pub fn set_word(&mut self, address: u16, value: i16) {
        let translate_address = Memory::translate_address(address).unwrap();
        match translate_address.section {
            Section::Rom => {
                self.rom[translate_address.address as usize] = (value >> 8) as u8;
                self.rom[translate_address.address as usize] = value as u8;
            }
        };
    }

    pub fn new(rom: Vec<u8>) -> Memory {
        Memory { rom }
    }
}
