use super::spell::Spell;
use super::wiz::{Acceptance, Affinity};
use std::io::Read;

use crate::generational_arena::Index;


#[derive(Clone, Debug)]
pub struct SpellBook {
    pub(in super::super) id: Option<Index>,
    pub(in super::super) spells: Vec<Spell>,
    pub(in super::super) glyphs: Affinity,
    pub(in super::super) style: Acceptance,
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
