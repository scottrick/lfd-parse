use core::fmt::Formatter;
use std::convert::From;
use std::fmt::Debug;

#[derive(PartialEq)]
pub enum LfdHeaderType {
    Anim,      // 2D Animations
    Blas,      // Creative .voc sound data, mostly weapons
    Bmap,      // ? Bitmap data
    Cust,      // ? Image data
    Delt,      // Image data
    Film,      // View layout
    Font,      // Font glyph
    Gmid,      // MIDI
    Mask,      // Cockpit transparency mask
    Mtrx,      // ? Matrix
    Panl,      // Cockpit panel image data
    Pltt,      // Palette
    Rmap,      // Initial LFD file header
    Ship,      // Craft vector data
    Text,      // Plain text
    Voic,      // Creative .voc sound data, mostly voice
    Xact,      // Explosions, backdrops, and others
    Unkn(u32), // Unknown
}

impl LfdHeaderType {
    pub fn to_u32(&self) -> u32 {
        match self {
            LfdHeaderType::Anim => 0x414E494D,
            LfdHeaderType::Blas => 0x424C4153,
            LfdHeaderType::Bmap => 0x424D4150,
            LfdHeaderType::Cust => 0x43555354,
            LfdHeaderType::Delt => 0x44454C54,
            LfdHeaderType::Film => 0x46494C4D,
            LfdHeaderType::Font => 0x464F4E54,
            LfdHeaderType::Gmid => 0x474D4944,
            LfdHeaderType::Mask => 0x4D41534B,
            LfdHeaderType::Mtrx => 0x4D545258,
            LfdHeaderType::Panl => 0x50414E4C,
            LfdHeaderType::Pltt => 0x504C5454,
            LfdHeaderType::Rmap => 0x524D4150,
            LfdHeaderType::Ship => 0x53484950,
            LfdHeaderType::Text => 0x54455854,
            LfdHeaderType::Voic => 0x564F4943,
            LfdHeaderType::Xact => 0x58414354,
            LfdHeaderType::Unkn(value) => *value,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LfdHeaderType::Anim => String::from("Anim"),
            LfdHeaderType::Blas => String::from("Blas"),
            LfdHeaderType::Bmap => String::from("Bmap"),
            LfdHeaderType::Cust => String::from("Cust"),
            LfdHeaderType::Delt => String::from("Delt"),
            LfdHeaderType::Film => String::from("Film"),
            LfdHeaderType::Font => String::from("Font"),
            LfdHeaderType::Gmid => String::from("Gmid"),
            LfdHeaderType::Mask => String::from("Mask"),
            LfdHeaderType::Mtrx => String::from("Mtrx"),
            LfdHeaderType::Panl => String::from("Panl"),
            LfdHeaderType::Pltt => String::from("Pltt"),
            LfdHeaderType::Rmap => String::from("Rmap"),
            LfdHeaderType::Ship => String::from("Ship"),
            LfdHeaderType::Text => String::from("Text"),
            LfdHeaderType::Voic => String::from("Voic"),
            LfdHeaderType::Xact => String::from("Xact"),
            LfdHeaderType::Unkn(value) => format!("Unkn {:#01X}", value),
        }
    }
}

impl From<u32> for LfdHeaderType {
    fn from(value: u32) -> Self {
        match value {
            0x414E494D => LfdHeaderType::Anim,
            0x424C4153 => LfdHeaderType::Blas,
            0x424D4150 => LfdHeaderType::Bmap,
            0x43555354 => LfdHeaderType::Cust,
            0x44454C54 => LfdHeaderType::Delt,
            0x46494C4D => LfdHeaderType::Film,
            0x464F4E54 => LfdHeaderType::Font,
            0x474D4944 => LfdHeaderType::Gmid,
            0x4D41534B => LfdHeaderType::Mask,
            0x4D545258 => LfdHeaderType::Mtrx,
            0x50414E4C => LfdHeaderType::Panl,
            0x504C5454 => LfdHeaderType::Pltt,
            0x524D4150 => LfdHeaderType::Rmap,
            0x53484950 => LfdHeaderType::Ship,
            0x54455854 => LfdHeaderType::Text,
            0x564F4943 => LfdHeaderType::Voic,
            0x58414354 => LfdHeaderType::Xact,
            _ => LfdHeaderType::Unkn(value),
        }
    }
}

impl Debug for LfdHeaderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
