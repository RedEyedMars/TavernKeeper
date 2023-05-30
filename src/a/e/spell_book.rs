use super::spell::Spell;
use super::wiz::{Acceptance, Affinity};
use super::{Glyph, Style};
use crate::generational_arena::Index;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SpellBook {
    id: Option<Index>,
    spells: Vec<Spell>,
    glyphs: Affinity,
    style: Acceptance,
}

impl SpellBook {
    pub fn new() -> Self {
        SpellBook {
            id: None,
            spells: vec![],
            glyphs: Affinity {
                fire: 0,
                air: 0,
                earth: 0,
                water: 0,
                void: 0,
            },
            style: Acceptance::new(),
        }
    }

    pub fn add_spell(&mut self, spell: Spell) {
        self.spells.push(spell);
    }

    pub fn glyphs(&self) -> &Affinity {
        &self.glyphs
    }

    pub fn style(&self) -> &Acceptance {
        &self.style
    }

    pub fn spells(&self) -> &Vec<Spell> {
        &self.spells
    }
}

impl PartialEq for SpellBook {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SpellBook {}
