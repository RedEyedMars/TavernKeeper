use crate::a::e::spell_book::SpellBook;
use generational_arena::Index;

use super::Glyph;
use super::Style;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum MindSet {
    Coward,
    Aggressive,
    Defensive,
    Neutral,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Affinity {
    pub fire: u32,
    pub air: u32,
    pub earth: u32,
    pub water: u32,
    pub void: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Acceptance {
    highest: Style,
    pub elder: u32,
    pub eldrich: u32,
    pub ancient: u32,
    pub arcane: u32,
    pub void: u32,
}

#[derive(Clone, Debug)]
pub struct Wizard {
    pub id: Option<Index>,
    pub name: String,
    selected_spellbook: usize,
    spellbooks: Vec<SpellBook>,
    pub state: MindSet,
    pub affinity: Affinity,
    pub acceptance: Acceptance,
}

impl Wizard {
    pub fn new(name: String) -> Wizard {
        Wizard {
            id: None,
            name,
            selected_spellbook: 0,
            spellbooks: vec![SpellBook::new()],
            state: MindSet::Neutral,
            affinity: Affinity::new(),
            acceptance: Acceptance::new(),
        }
    }
}

impl Wizard {}

impl Affinity {
    pub fn new() -> Self {
        Affinity {
            fire: 0,
            air: 0,
            earth: 0,
            water: 0,
            void: 0,
        }
    }
    pub fn val(&self, glyph: &Glyph) -> u32 {
        match glyph {
            Glyph::Fire => self.fire,
            Glyph::Air => self.air,
            Glyph::Earth => self.earth,
            Glyph::Water => self.water,
            Glyph::Void => self.void,
        }
    }
}

impl Acceptance {
    pub fn new() -> Self {
        Acceptance {
            elder: 0,
            eldrich: 0,
            ancient: 0,
            arcane: 0,
            void: 0,
            highest: Style::Void,
        }
    }

    pub fn from_style(style: Style, val: u32) -> Self {
        let mut acceptance = Acceptance::new();
        match style {
            Style::Elder => acceptance.elder = val,
            Style::Eldrich => acceptance.eldrich = val,
            Style::Ancient => acceptance.ancient = val,
            Style::Arcane => acceptance.arcane = val,
            Style::Void => acceptance.void = val,
        }
        acceptance.highest = style;
        acceptance
    }
    pub fn val(&self, style: &Style) -> u32 {
        match style {
            Style::Elder => self.elder,
            Style::Eldrich => self.eldrich,
            Style::Ancient => self.ancient,
            Style::Arcane => self.arcane,
            Style::Void => self.void,
        }
    }
    pub fn add(&mut self, acceptance: &Acceptance) {
        self.elder += acceptance.elder;
        self.eldrich += acceptance.eldrich;
        self.ancient += acceptance.ancient;
        self.arcane += acceptance.arcane;
        self.void += acceptance.void;

        self.reevaluate();
    }

    fn reevaluate(&mut self) {
        if self.elder > self.val(&self.highest) {
            self.highest = Style::Elder;
        } else if self.eldrich > self.val(&self.highest) {
            self.highest = Style::Eldrich;
        } else if self.ancient > self.val(&self.highest) {
            self.highest = Style::Ancient;
        } else if self.arcane > self.val(&self.highest) {
            self.highest = Style::Arcane;
        } else if self.void > self.val(&self.highest) {
            self.highest = Style::Void;
        }
    }

    pub fn get_highest(&self) -> &Style {
        &self.highest
    }
}

impl PartialEq for Wizard {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Wizard {}
