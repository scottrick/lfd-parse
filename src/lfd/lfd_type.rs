#[derive(Debug)]
pub enum LfdHeaderType {
    Anim, // 2D Animations
    Blas, // Creative .voc sound data, mostly weapons
    Bmap, // ? Bitmap data
    Cust, // ? Image data
    Delt, // Image data
    Film, // View layout
    Font, // Font glyph
    Gmid, // MIDI
    Mask, // Cockpit transparency mask
    Mtrx, // ? Matrix
    Panl, // Cockpit panel image data
    Pltt, // Palette
    Rmap, // Initial LFD file header
    Ship, // Craft vector data
    Text, // Plain text
    Voic, // Creative .voc sound data, mostly voice
    Xact, // Explosions, backdrops, and others
    Unkn, // Unknown
}

impl LfdHeaderType {
    pub fn from_u32(value: u32) -> Self {
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
            _ => LfdHeaderType::Unkn,
        }
    }
}
