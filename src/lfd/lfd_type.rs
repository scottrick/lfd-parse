use core::fmt::Formatter;
use std::convert::From;
use std::fmt::Debug;

pub enum LfdHeaderType {
    Anim(u32), // 2D Animations
    Blas(u32), // Creative .voc sound data, mostly weapons
    Bmap(u32), // ? Bitmap data
    Cust(u32), // ? Image data
    Delt(u32), // Image data
    Film(u32), // View layout
    Font(u32), // Font glyph
    Gmid(u32), // MIDI
    Mask(u32), // Cockpit transparency mask
    Mtrx(u32), // ? Matrix
    Panl(u32), // Cockpit panel image data
    Pltt(u32), // Palette
    Rmap(u32), // Initial LFD file header
    Ship(u32), // Craft vector data
    Text(u32), // Plain text
    Voic(u32), // Creative .voc sound data, mostly voice
    Xact(u32), // Explosions, backdrops, and others
    Unkn(u32), // Unknown
}

impl LfdHeaderType {
    pub fn to_u32(&self) -> u32 {
        match self {
            LfdHeaderType::Anim(value) => *value,
            LfdHeaderType::Blas(value) => *value,
            LfdHeaderType::Bmap(value) => *value,
            LfdHeaderType::Cust(value) => *value,
            LfdHeaderType::Delt(value) => *value,
            LfdHeaderType::Film(value) => *value,
            LfdHeaderType::Font(value) => *value,
            LfdHeaderType::Gmid(value) => *value,
            LfdHeaderType::Mask(value) => *value,
            LfdHeaderType::Mtrx(value) => *value,
            LfdHeaderType::Panl(value) => *value,
            LfdHeaderType::Pltt(value) => *value,
            LfdHeaderType::Rmap(value) => *value,
            LfdHeaderType::Ship(value) => *value,
            LfdHeaderType::Text(value) => *value,
            LfdHeaderType::Voic(value) => *value,
            LfdHeaderType::Xact(value) => *value,
            LfdHeaderType::Unkn(value) => *value,
        }
    }
}

impl From<u32> for LfdHeaderType {
    fn from(value: u32) -> Self {
        match value {
            0x414E494D => LfdHeaderType::Anim(value),
            0x424C4153 => LfdHeaderType::Blas(value),
            0x424D4150 => LfdHeaderType::Bmap(value),
            0x43555354 => LfdHeaderType::Cust(value),
            0x44454C54 => LfdHeaderType::Delt(value),
            0x46494C4D => LfdHeaderType::Film(value),
            0x464F4E54 => LfdHeaderType::Font(value),
            0x474D4944 => LfdHeaderType::Gmid(value),
            0x4D41534B => LfdHeaderType::Mask(value),
            0x4D545258 => LfdHeaderType::Mtrx(value),
            0x50414E4C => LfdHeaderType::Panl(value),
            0x504C5454 => LfdHeaderType::Pltt(value),
            0x524D4150 => LfdHeaderType::Rmap(value),
            0x53484950 => LfdHeaderType::Ship(value),
            0x54455854 => LfdHeaderType::Text(value),
            0x564F4943 => LfdHeaderType::Voic(value),
            0x58414354 => LfdHeaderType::Xact(value),
            _ => LfdHeaderType::Unkn(value),
        }
    }
}

impl Debug for LfdHeaderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LfdHeaderType::Anim(_) => String::from("Anim"),
            LfdHeaderType::Blas(_) => String::from("Blas"),
            LfdHeaderType::Bmap(_) => String::from("Bmap"),
            LfdHeaderType::Cust(_) => String::from("Cust"),
            LfdHeaderType::Delt(_) => String::from("Delt"),
            LfdHeaderType::Film(_) => String::from("Film"),
            LfdHeaderType::Font(_) => String::from("Font"),
            LfdHeaderType::Gmid(_) => String::from("Gmid"),
            LfdHeaderType::Mask(_) => String::from("Mask"),
            LfdHeaderType::Mtrx(_) => String::from("Mtrx"),
            LfdHeaderType::Panl(_) => String::from("Panl"),
            LfdHeaderType::Pltt(_) => String::from("Pltt"),
            LfdHeaderType::Rmap(_) => String::from("Rmap"),
            LfdHeaderType::Ship(_) => String::from("Ship"),
            LfdHeaderType::Text(_) => String::from("Text"),
            LfdHeaderType::Voic(_) => String::from("Voic"),
            LfdHeaderType::Xact(_) => String::from("Xact"),
            LfdHeaderType::Unkn(value) => format!("Unkn {:#01X}", value),
        };

        f.write_str(&msg)
    }
}
