use crate::a::e::spell_book::SpellBook;
use generational_arena::Index;

use super::Glyph;
use super::Style;
use super::spell::Spell;
use super::spell::Status;

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
    pub hp: u32,
    pub max_hp: u32,
    pub status: StatusSet,
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

    pub fn as_output(&self, battles: Vec<Index>) -> std::io::Result<Vec<u8>> {
        use byteorder::{LittleEndian, WriteBytesExt};
        let name_as_bytes = self.name.as_bytes();
        let acceptance_as_bytes = self.acceptance.as_output();
        let affinity_as_bytes = self.affinity.as_output();
        let status_as_bytes = self.status.as_output()?;
        let mut spellbooks_as_bytes: Vec<u8> = Vec::new();
        for spellbook in self.spellbooks.iter() {
            spellbooks_as_bytes.extend(spellbook.as_output()?);
        }
        let mut output = Vec::with_capacity(
            std::mem::size_of::<usize>() + // name len
            name_as_bytes.len() + // name
            std::mem::size_of::<u32>() + // hp
            std::mem::size_of::<u32>() + // max_hp
            acceptance_as_bytes.len() + // acceptance
            affinity_as_bytes.len() + // affinity
            status_as_bytes.len() + // status
            std::mem::size_of::<usize>() + // selected_spellbook
            std::mem::size_of::<usize>() + // spellbooks.len()
            spellbooks_as_bytes.len() + // spellbooks
            std::mem::size_of::<usize>() + // battles.len()
            std::mem::size_of::<usize>() * battles.len());
        output.extend_from_slice(&name_as_bytes.len().to_le_bytes());
        output.extend(name_as_bytes);
        output.write_u32::<LittleEndian>(self.hp)?;
        output.write_u32::<LittleEndian>(self.max_hp)?;
        output.extend(acceptance_as_bytes);
        output.extend(affinity_as_bytes);
        output.extend(status_as_bytes);
        output.extend_from_slice(&self.selected_spellbook.to_le_bytes());
        output.extend_from_slice(&self.spellbooks.len().to_le_bytes());
        output.extend(spellbooks_as_bytes);
        output.extend_from_slice(&battles.len().to_le_bytes());
        for battle in battles.into_iter() {
            output.extend_from_slice(&battle.into_raw_parts().0.to_le_bytes());
        }
        Ok(output)
    }

    pub fn from_buf(buf: &[u8]) -> std::io::Result<Wizard> {
        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::{Read, Cursor};
        let mut buf = Cursor::new(buf);
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf).expect("Failed to read wizard name size");
        let name_len = usize::from_le_bytes(usize_buf);
        let mut name_buf = vec![0u8; name_len];
        buf.read_exact(&mut name_buf).expect("Failed to read wizard name");
        let name = String::from_utf8(name_buf).expect("Failed to parse wizard name");
        let hp = buf.read_u32::<LittleEndian>()?;
        let max_hp = buf.read_u32::<LittleEndian>()?;
        let acceptance = Acceptance::from_buf(&mut buf)?;
        let affinity = Affinity::from_buf(&mut buf)?;
        let status = StatusSet::from_buf(&mut buf)?;

        buf.read_exact(&mut usize_buf).expect("Failed to read wizard name size");
        let selected_spellbook = usize::from_le_bytes(usize_buf);
        let spellbooks = SpellBook::from_buf(&mut buf)?;

        Ok(Wizard {
            id: None,
            name,
            hp,
            max_hp,
            status,
            selected_spellbook,
            spellbooks,
            state: MindSet::Neutral,
            affinity,
            acceptance,
        })
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

    pub fn as_output(&self) -> Vec<u8> {
        use byteorder::{LittleEndian, WriteBytesExt};
        let mut output = Vec::with_capacity(4*5);
        output.write_u32::<LittleEndian>(self.fire).unwrap();
        output.write_u32::<LittleEndian>(self.air).unwrap();
        output.write_u32::<LittleEndian>(self.earth).unwrap();
        output.write_u32::<LittleEndian>(self.water).unwrap();
        output.write_u32::<LittleEndian>(self.void).unwrap();
        output
    }

    pub fn from_buf(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        use byteorder::{LittleEndian, ReadBytesExt};
        Ok(Affinity {
            fire: buf.read_u32::<LittleEndian>()?,
            air: buf.read_u32::<LittleEndian>()?,
            earth: buf.read_u32::<LittleEndian>()?,
            water: buf.read_u32::<LittleEndian>()?,
            void: buf.read_u32::<LittleEndian>()?,
        })
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

    pub fn as_output(&self) -> Vec<u8> {
        use byteorder::{LittleEndian, WriteBytesExt};
        let mut output = Vec::with_capacity(4*5);
        output.write_u32::<LittleEndian>(self.elder).unwrap();
        output.write_u32::<LittleEndian>(self.eldrich).unwrap();
        output.write_u32::<LittleEndian>(self.ancient).unwrap();
        output.write_u32::<LittleEndian>(self.arcane).unwrap();
        output.write_u32::<LittleEndian>(self.void).unwrap();
        output
    }

    pub fn from_buf(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        use byteorder::{LittleEndian, ReadBytesExt};
        let mut result = Acceptance {
            elder: buf.read_u32::<LittleEndian>()?,
            eldrich: buf.read_u32::<LittleEndian>()?,
            ancient: buf.read_u32::<LittleEndian>()?,
            arcane: buf.read_u32::<LittleEndian>()?,
            void: buf.read_u32::<LittleEndian>()?,
            highest: Style::Void,
        };
        result.reevaluate();
        Ok(result)
    }
}

impl PartialEq for Wizard {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Wizard {}

#[derive(Clone, Debug)]
pub struct StatusSet {
    barrier_fire: (u16, u16),
    barrier_air: (u16, u16),
    barrier_earth: (u16, u16),
    barrier_water: (u16, u16),
    barrier_void: (u16, u16),
    burning: (u16, u16),
    stunned: (u16, u16),
    submerged: (u16, u16),
    shocked: (u16, u16),
    weakened: (u16, u16),
    raging: (u16, u16),
    hardened: (u16, u16),
    fluid: (u16, u16),
    flying: (u16, u16),
}

impl StatusSet {
    pub fn new() -> Self {
        StatusSet {
            barrier_fire: (0, 0),
            barrier_air: (0, 0),
            barrier_earth: (0, 0),
            barrier_water: (0, 0),
            barrier_void: (0, 0),
            burning: (0, 0),
            stunned: (0, 0),
            submerged: (0, 0),
            shocked: (0, 0),
            weakened: (0, 0),
            raging: (0, 0),
            hardened: (0, 0),
            fluid: (0, 0),
            flying: (0, 0),
        }
    }

    pub fn insert(&mut self, status: &Status, value: u16, duration: u16) {
        match status {
            Status::Barrier(Glyph::Fire) => {
                if value * duration > self.barrier_fire.0 * self.barrier_fire.1 {
                    self.barrier_fire = (value, duration)
                }
            },
            Status::Barrier(Glyph::Air) => {
                if value * duration > self.barrier_air.0 * self.barrier_air.1 {
                    self.barrier_air = (value, duration)
                }
            },
            Status::Barrier(Glyph::Earth) => {
                if value * duration > self.barrier_earth.0 * self.barrier_earth.1 {
                    self.barrier_earth = (value, duration)
                }
            },
            Status::Barrier(Glyph::Water) => {
                if value * duration > self.barrier_water.0 * self.barrier_water.1 {
                    self.barrier_water = (value, duration)
                }
            },
            Status::Barrier(Glyph::Void) => {
                if value * duration > self.barrier_void.0 * self.barrier_void.1 {
                    self.barrier_void = (value, duration)
                }
            },
            Status::Burning => {
                if value * duration > self.burning.0 * self.burning.1 {
                    self.burning = (value, duration)
                }
            },
            Status::Stunned => {
                if value * duration > self.stunned.0 * self.stunned.1 {
                    self.stunned = (value, duration)
                }
            },
            Status::Submerged => {
                if value * duration > self.submerged.0 * self.submerged.1 {
                    self.submerged = (value, duration)
                }
            },
            Status::Shocked => {
                if value * duration > self.shocked.0 * self.shocked.1 {
                    self.shocked = (value, duration)
                }
            },
            Status::Weakened => {
                if value * duration > self.weakened.0 * self.weakened.1 {
                    self.weakened = (value, duration)
                }
            },
            Status::Raging => {
                if value * duration > self.raging.0 * self.raging.1 {
                    self.raging = (value, duration)
                }
            },
            Status::Hardened => {
                if value * duration > self.hardened.0 * self.hardened.1 {
                    self.hardened = (value, duration)
                }
            },
            Status::Fluid => {
                if value * duration > self.fluid.0 * self.fluid.1 {
                    self.fluid = (value, duration)
                }
            },
            Status::Flying => {
                if value * duration > self.flying.0 * self.flying.1 {
                    self.flying = (value, duration)
                }
            },
        }
    }

    pub fn value(&self, status: &Status) -> u16 {
        match status {
            Status::Barrier(Glyph::Fire) => self.barrier_fire.0,
            Status::Barrier(Glyph::Air) => self.barrier_air.0,
            Status::Barrier(Glyph::Earth) => self.barrier_earth.0,
            Status::Barrier(Glyph::Water) => self.barrier_water.0,
            Status::Barrier(Glyph::Void) => self.barrier_void.0,
            Status::Burning => self.burning.0,
            Status::Stunned => self.stunned.0,
            Status::Submerged => self.submerged.0,
            Status::Shocked => self.shocked.0,
            Status::Weakened => self.weakened.0,
            Status::Raging => self.raging.0,
            Status::Hardened => self.hardened.0,
            Status::Fluid => self.fluid.0,
            Status::Flying => self.flying.0,
        }
    }

    pub fn entry(&self, status: &Status) -> Option<u16> {
        let val = self.value(status);
        if val == 0 {
            None
        } else {
            Some(val)
        }
    }

    pub fn duration(&self, status: &Status) -> u16 {
        match status {
            Status::Barrier(Glyph::Fire) => self.barrier_fire.1,
            Status::Barrier(Glyph::Air) => self.barrier_air.1,
            Status::Barrier(Glyph::Earth) => self.barrier_earth.1,
            Status::Barrier(Glyph::Water) => self.barrier_water.1,
            Status::Barrier(Glyph::Void) => self.barrier_void.1,
            Status::Burning => self.burning.1,
            Status::Stunned => self.stunned.1,
            Status::Submerged => self.submerged.1,
            Status::Shocked => self.shocked.1,
            Status::Weakened => self.weakened.1,
            Status::Raging => self.raging.1,
            Status::Hardened => self.hardened.1,
            Status::Fluid => self.fluid.1,
            Status::Flying => self.flying.1,
        }
    }

    pub fn remove(&mut self, status: &Status) {
        match status {
            Status::Barrier(Glyph::Fire) => self.barrier_fire = (0, 0),
            Status::Barrier(Glyph::Air) => self.barrier_air = (0, 0),
            Status::Barrier(Glyph::Earth) => self.barrier_earth = (0, 0),
            Status::Barrier(Glyph::Water) => self.barrier_water = (0, 0),
            Status::Barrier(Glyph::Void) => self.barrier_void = (0, 0),
            Status::Burning => self.burning = (0, 0),
            Status::Stunned => self.stunned = (0, 0),
            Status::Submerged => self.submerged = (0, 0),
            Status::Shocked => self.shocked = (0, 0),
            Status::Weakened => self.weakened = (0, 0),
            Status::Raging => self.raging = (0, 0),
            Status::Hardened => self.hardened = (0, 0),
            Status::Fluid => self.fluid = (0, 0),
            Status::Flying => self.flying = (0, 0),
        }
    }

    pub fn has(&self, status: &Status) -> bool {
        match status {
            Status::Barrier(Glyph::Fire) => self.barrier_fire.1 != 0,
            Status::Barrier(Glyph::Air) => self.barrier_air.1 != 0,
            Status::Barrier(Glyph::Earth) => self.barrier_earth.1 != 0,
            Status::Barrier(Glyph::Water) => self.barrier_water.1 != 0,
            Status::Barrier(Glyph::Void) => self.barrier_void.1 != 0,
            Status::Burning => self.burning.1 != 0,
            Status::Stunned => self.stunned.1 != 0,
            Status::Submerged => self.submerged.1 != 0,
            Status::Shocked => self.shocked.1 != 0,
            Status::Weakened => self.weakened.1 != 0,
            Status::Raging => self.raging.1 != 0,
            Status::Hardened => self.hardened.1 != 0,
            Status::Fluid => self.fluid.1 != 0,
            Status::Flying => self.flying.1 != 0,
        }
    }

    pub fn tick(&mut self, status: &Status) -> bool {
        match status {
            Status::Barrier(Glyph::Fire) => {
                if self.barrier_fire.1 == 0 {
                    return false;
                }
                self.barrier_fire.1 -= 1;
                self.barrier_fire.1 != 0
            }
            Status::Barrier(Glyph::Air) => {
                if self.barrier_air.1 == 0 {
                    return false;
                }
                self.barrier_air.1 -= 1;
                self.barrier_air.1 != 0
            }
            Status::Barrier(Glyph::Earth) => {
                if self.barrier_earth.1 == 0 {
                    return false;
                }
                self.barrier_earth.1 -= 1;
                self.barrier_earth.1 != 0
            }
            Status::Barrier(Glyph::Water) => {
                if self.barrier_water.1 == 0 {
                    return false;
                }
                self.barrier_water.1 -= 1;
                self.barrier_water.1 != 0
            }
            Status::Barrier(Glyph::Void) => {
                if self.barrier_void.1 == 0 {
                    return false;
                }
                self.barrier_void.1 -= 1;
                self.barrier_void.1 != 0
            }
            Status::Burning => {
                if self.burning.1 == 0 {
                    return false;
                }
                self.burning.1 -= 1;
                self.burning.1 != 0
            }
            Status::Stunned => {
                if self.stunned.1 == 0 {
                    return false;
                }
                self.stunned.1 -= 1;
                self.stunned.1 != 0
            }
            Status::Submerged => {
                if self.submerged.1 == 0 {
                    return false;
                }
                self.submerged.1 -= 1;
                self.submerged.1 != 0
            }
            Status::Shocked => {
                if self.shocked.1 == 0 {
                    return false;
                }
                self.shocked.1 -= 1;
                self.shocked.1 != 0
            }
            Status::Weakened => {
                if self.weakened.1 == 0 {
                    return false;
                }
                self.weakened.1 -= 1;
                self.weakened.1 != 0
            }
            Status::Raging => {
                if self.raging.1 == 0 {
                    return false;
                }
                self.raging.1 -= 1;
                self.raging.1 != 0
            }
            Status::Hardened => {
                if self.hardened.1 == 0 {
                    return false;
                }
                self.hardened.1 -= 1;
                self.hardened.1 != 0
            }
            Status::Fluid => {
                if self.fluid.1 == 0 {
                    return false;
                }
                self.fluid.1 -= 1;
                self.fluid.1 != 0
            }
            Status::Flying => {
                if self.flying.1 == 0 {
                    return false;
                }
                self.flying.1 -= 1;
                self.flying.1 != 0
            }
        }
    }

    pub fn tick_all(&mut self) {
        if self.barrier_fire.1 > 0 {
            self.barrier_fire.1 -= 1;
        }
        if self.barrier_air.1 > 0 {
            self.barrier_air.1 -= 1;
        }
        if self.barrier_earth.1 > 0 {
            self.barrier_earth.1 -= 1;
        }
        if self.barrier_water.1 > 0 {
            self.barrier_water.1 -= 1;
        }
        if self.barrier_void.1 > 0 {
            self.barrier_void.1 -= 1;
        }
        if self.burning.1 > 0 {
            self.burning.1 -= 1;
        }
        if self.stunned.1 > 0 {
            self.stunned.1 -= 1;
        }
        if self.submerged.1 > 0 {
            self.submerged.1 -= 1;
        }
        if self.shocked.1 > 0 {
            self.shocked.1 -= 1;
        }
        if self.weakened.1 > 0 {
            self.weakened.1 -= 1;
        }
        if self.raging.1 > 0 {
            self.raging.1 -= 1;
        }
        if self.hardened.1 > 0 {
            self.hardened.1 -= 1;
        }
        if self.fluid.1 > 0 {
            self.fluid.1 -= 1;
        }
        if self.flying.1 > 0 {
            self.flying.1 -= 1;
        }
    }

    pub fn as_output(&self) -> std::io::Result<Vec<u8>> {
        use byteorder::{LittleEndian, WriteBytesExt};
        let mut output = Vec::with_capacity(4*14);
        output.write_u16::<LittleEndian>(self.barrier_fire.0)?;
        output.write_u16::<LittleEndian>(self.barrier_fire.1)?;
        output.write_u16::<LittleEndian>(self.barrier_air.0)?;
        output.write_u16::<LittleEndian>(self.barrier_air.1)?;
        output.write_u16::<LittleEndian>(self.barrier_earth.0)?;
        output.write_u16::<LittleEndian>(self.barrier_earth.1)?;
        output.write_u16::<LittleEndian>(self.barrier_water.0)?;
        output.write_u16::<LittleEndian>(self.barrier_water.1)?;
        output.write_u16::<LittleEndian>(self.barrier_void.0)?;
        output.write_u16::<LittleEndian>(self.barrier_void.1)?;
        output.write_u16::<LittleEndian>(self.burning.0)?;
        output.write_u16::<LittleEndian>(self.burning.1)?;
        output.write_u16::<LittleEndian>(self.stunned.0)?;
        output.write_u16::<LittleEndian>(self.stunned.1)?;
        output.write_u16::<LittleEndian>(self.submerged.0)?;
        output.write_u16::<LittleEndian>(self.submerged.1)?;
        output.write_u16::<LittleEndian>(self.shocked.0)?;
        output.write_u16::<LittleEndian>(self.shocked.1)?;
        output.write_u16::<LittleEndian>(self.weakened.0)?;
        output.write_u16::<LittleEndian>(self.weakened.1)?;
        output.write_u16::<LittleEndian>(self.raging.0)?;
        output.write_u16::<LittleEndian>(self.raging.1)?;
        output.write_u16::<LittleEndian>(self.hardened.0)?;
        output.write_u16::<LittleEndian>(self.hardened.1)?;
        output.write_u16::<LittleEndian>(self.fluid.0)?;
        output.write_u16::<LittleEndian>(self.fluid.1)?;
        output.write_u16::<LittleEndian>(self.flying.0)?;
        output.write_u16::<LittleEndian>(self.flying.1)?;
        Ok(output)
    }

    pub fn from_buf(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        use byteorder::{LittleEndian, ReadBytesExt};
        Ok(StatusSet {
            barrier_fire: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            barrier_air: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            barrier_earth: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            barrier_water: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            barrier_void: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            burning: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            stunned: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            submerged: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            shocked: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            weakened: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            raging: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            hardened: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            fluid: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
            flying: (buf.read_u16::<LittleEndian>()?, buf.read_u16::<LittleEndian>()?),
        })
    }
}

impl PartialEq for StatusSet {
    fn eq(&self, other: &Self) -> bool {
        (self.barrier_fire.0 == 0 && other.barrier_fire.0 == 0 || self.barrier_fire.0 > 0 && other.barrier_fire.0 > 0)
        && (self.barrier_air.0 == 0 && other.barrier_air.0 == 0 || self.barrier_air.0 > 0 && other.barrier_air.0 > 0)
        && (self.barrier_earth.0 == 0 && other.barrier_earth.0 == 0 || self.barrier_earth.0 > 0 && other.barrier_earth.0 > 0)
        && (self.barrier_water.0 == 0 && other.barrier_water.0 == 0 || self.barrier_water.0 > 0 && other.barrier_water.0 > 0)
        && (self.barrier_void.0 == 0 && other.barrier_void.0 == 0 || self.barrier_void.0 > 0 && other.barrier_void.0 > 0)
        && (self.burning.0 == 0 && other.burning.0 == 0 || self.burning.0 > 0 && other.burning.0 > 0)
        && (self.stunned.0 == 0 && other.stunned.0 == 0 || self.stunned.0 > 0 && other.stunned.0 > 0)
        && (self.submerged.0 == 0 && other.submerged.0 == 0 || self.submerged.0 > 0 && other.submerged.0 > 0)
        && (self.shocked.0 == 0 && other.shocked.0 == 0 || self.shocked.0 > 0 && other.shocked.0 > 0)
        && (self.weakened.0 == 0 && other.weakened.0 == 0 || self.weakened.0 > 0 && other.weakened.0 > 0)
        && (self.raging.0 == 0 && other.raging.0 == 0 || self.raging.0 > 0 && other.raging.0 > 0)
        && (self.hardened.0 == 0 && other.hardened.0 == 0 || self.hardened.0 > 0 && other.hardened.0 > 0)
        && (self.fluid.0 == 0 && other.fluid.0 == 0 || self.fluid.0 > 0 && other.fluid.0 > 0)
        && (self.flying.0 == 0 && other.flying.0 == 0 || self.flying.0 > 0 && other.flying.0 > 0)
    }
}

impl Eq for StatusSet {}