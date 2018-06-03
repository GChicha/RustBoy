pub struct Memory {
    rom : Vec<u8>,
}

enum Section {
    Rom,
}

struct TranslatedAddress {
    section : Section,
    address : usize,
}

impl Memory {
    fn translate_address(address : usize)
        -> Result<TranslatedAddress, &'static str> {
        if address == 0 {
            Err("Memory must be higher than zero")
        } else if address > 0xffff {
            Err("Memory must be lower than 0xFFFF")
        } else if address < 0x8000 {
            Ok(TranslatedAddress {section : Section::Rom, address : address})
        } else {
            Err("Not mapped yet")
        }
    }

    pub fn get_byte(&self, address : usize)
        -> Result<u8, &'static str> {
        let translate_address = Memory::translate_address(address)?;
        match translate_address.section {
            Section::Rom => Ok(self.rom[translate_address.address]),
            _ => Err("Not mapped yet"),
        }
    }

    pub fn get_word(&self, address : usize)
        -> Result<u16, &'static str> {
        let translate_address = Memory::translate_address(address)?;
        match translate_address.section {
            Section::Rom => {
                let data = self.rom[translate_address.address] as u16;
                let data = (data << 8) | self.rom[translate_address.address + 1] as u16;

                Ok(data)
            },
            _ => Err("Not mapped yet"),
        }
    }

    pub fn new(rom : Vec<u8>) -> Memory {
        Memory {rom}
    }
}
