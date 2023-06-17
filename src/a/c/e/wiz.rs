use super::spell_book::SpellBook;
use generational_arena::Index;

use super::Glyph;
use super::Style;
use super::spell::Spell;
use super::status::StatusSet;

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
    pub(in super::super) highest: Style,
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
    pub hp: u32,
    pub max_hp: u32,
    pub status: StatusSet,
    pub(in super::super)selected_spellbook: usize,
    pub(in super::super)spellbooks: Vec<SpellBook>,
    pub state: MindSet,
    pub affinity: Affinity,
    pub acceptance: Acceptance,
}

impl Wizard {
    pub fn new(name: String) -> Wizard {
        Wizard {
            id: None,
            name,
            hp: 100,
            max_hp: 100,
            status: StatusSet::new(),
            selected_spellbook: 0,
            spellbooks: vec![SpellBook::new()],
            state: MindSet::Neutral,
            affinity: Affinity::new(),
            acceptance: Acceptance::new(),
        }
    }

    pub fn get_spells(&self) -> &Vec<Spell> {
        self.spellbooks[self.selected_spellbook].spells()
    }

    pub fn augment(&self, glyph: &Glyph) -> u16 {
        self.affinity.val(glyph) as u16
    }

    pub fn resist(&self, glyph: &Glyph) -> u16 {
        self.affinity.val(glyph) as u16
    }

    pub fn spellbook_augment(&self, glyph: &Glyph) -> u16 {
        self.spellbooks[self.selected_spellbook].glyphs().val(glyph) as u16
    }

    pub fn spellbook_affinity(&self) -> &Affinity {
        self.spellbooks[self.selected_spellbook].glyphs()
    }

    pub fn add_spell_to_book(&mut self, spell: Spell) {
        self.spellbooks[self.selected_spellbook].add_spell(spell);
    }
}

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

    pub fn val16(&self, glyph: &Glyph) -> u16 {
        match glyph {
            Glyph::Fire => self.fire as u16,
            Glyph::Air => self.air as u16,
            Glyph::Earth => self.earth as u16,
            Glyph::Water => self.water as u16,
            Glyph::Void => self.void as u16,
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

    pub(in super::super) fn reevaluate(&mut self) {
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
