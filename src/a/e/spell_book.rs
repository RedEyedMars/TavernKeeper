use super::spell::Spell;
use super::wiz::{Acceptance, Affinity};
use std::io::Read;

use crate::generational_arena::Index;


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

    pub fn as_output(&self) -> std::io::Result<Vec<u8>> {
        let mut output = Vec::new(); 
        output.extend(self.glyphs.as_output());
        output.extend(self.style.as_output());
        output.extend_from_slice(&self.spells.len().to_le_bytes());
        for spell in &self.spells {
            output.extend(spell.as_output()?);
        }
        Ok(output)
    }

    pub fn from_buf(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Vec<Self>> {
        let mut spell_books = Vec::new();
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf)?;
        let number_of_spell_books = usize::from_le_bytes(usize_buf);
        for _ in 0..number_of_spell_books {
            let glyphs = Affinity::from_buf(buf)?;
            let style = Acceptance::from_buf(buf)?;
            let mut spells = Vec::new();
            buf.read_exact(&mut usize_buf)?;
            let number_of_spells = usize::from_le_bytes(usize_buf);
            for _ in 0..number_of_spells {
                spells.push(Spell::from_buf(buf)?);
            }
            spell_books.push(SpellBook {
                id: None,
                spells,
                glyphs,
                style,
            });
        }
        Ok(spell_books)
    }
}

impl PartialEq for SpellBook {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SpellBook {}
