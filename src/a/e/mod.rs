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

pub mod mon;
pub mod spell;
pub mod spell_book;
pub mod wiz;
