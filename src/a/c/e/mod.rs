#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Glyph {
    Fire,
    Water,
    Earth,
    Air,
    Void,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Style {
    Elder,
    Arcane,
    Ancient,
    Eldrich,
    Void,
}

impl Glyph {
    pub fn as_u8(&self) -> u8 {
        match self {
            Glyph::Fire => 0,
            Glyph::Water => 1,
            Glyph::Earth => 2,
            Glyph::Air => 3,
            Glyph::Void => 4,
        }
    }
    pub fn from_u8(byte: u8) -> Self {
        match byte {
            0 => Glyph::Fire,
            1 => Glyph::Water,
            2 => Glyph::Earth,
            3 => Glyph::Air,
            4 => Glyph::Void,
            _ => panic!("Invalid glyph byte: {}", byte),
        }
    }
}

impl Style {
    pub fn as_u8(&self) -> u8 {
        match self {
            Style::Elder => 0,
            Style::Arcane => 1,
            Style::Ancient => 2,
            Style::Eldrich => 3,
            Style::Void => 4,
        }
    }
    pub fn from_u8(byte: u8) -> Self {
        match byte {
            0 => Style::Elder,
            1 => Style::Arcane,
            2 => Style::Ancient,
            3 => Style::Eldrich,
            4 => Style::Void,
            _ => panic!("Invalid style byte: {}", byte),
        }
    }
}

pub mod status;
pub mod mon;
pub mod spell;
pub mod spell_book;
pub mod wiz;
pub mod party;

