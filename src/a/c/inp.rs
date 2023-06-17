
use std::io::Result;
use std::io::Cursor;

use crate::a::c::e::Glyph;
use crate::a::c::e::Style;
use crate::a::c::e::spell::Ability;
use crate::a::c::e::spell::EffectApplication;
use crate::a::c::e::spell::EffectDuration;
use crate::a::c::e::spell::Spell;
use crate::a::c::e::spell::spells;
use crate::a::c::e::spell_book::SpellBook;
use crate::a::c::e::status::StatusSet;
use crate::a::c::e::wiz::MindSet;
use crate::a::q::battle::Battle;
use crate::a::q::battle::BattleAtom;
use crate::a::q::battle::BattleEvent;
use crate::a::q::battle::BattleMut;

use super::e::mon::Monster;
use super::e::mon::MonsterType;
use super::e::party::Party;
use super::e::spell::Effect;
use super::e::spell::EffectProgression;
use super::e::spell::PriorityTypes;
use super::e::spell::TargetType;
use super::e::status::Status;
use super::e::wiz::Acceptance;
use super::e::wiz::Affinity;
use super::e::wiz::Wizard;
use byteorder::{LittleEndian, ReadBytesExt};
use uuid::Uuid;
use std::collections::HashMap;
use std::io::Read;
pub(in super) trait Inputable<T> {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<T>;
}

impl<T> Inputable<Vec<T>> for Vec<T> where T: Inputable<T> {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf)?;
        let count = usize::from_le_bytes(usize_buf);
        for _ in 0..count {
            result.push(T::from_bytes(buf)?);
        }
        Ok(result)
    }
}

impl<T,U> Inputable<HashMap<T,U>> for HashMap<T,U> where T: Inputable<T> + std::cmp::Eq + std::hash::Hash, U: Inputable<U> {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<HashMap<T,U>> {
        let mut result = HashMap::new();
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf)?;
        let count = usize::from_le_bytes(usize_buf);
        for _ in 0..count {
            result.insert(T::from_bytes(buf)?, U::from_bytes(buf)?);
        }
        Ok(result)
    }
}

impl Inputable<String> for String {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<String> {
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf)?;
        let len = usize::from_le_bytes(usize_buf);
        let mut string_buf = vec![0u8; len];
        buf.read_exact(&mut string_buf)?;
        Ok(String::from_utf8(string_buf).expect("Could not read String"))
    }
}

impl Inputable<usize> for usize {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<usize> {
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf)?;
        Ok(usize::from_le_bytes(usize_buf))
    }
}

impl Inputable<Wizard> for Wizard {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Wizard> {
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf).expect("Failed to read wizard name size");
        let name_len = usize::from_le_bytes(usize_buf);
        let mut name_buf = vec![0u8; name_len];
        buf.read_exact(&mut name_buf).expect("Failed to read wizard name");
        let name = String::from_utf8(name_buf).expect("Failed to parse wizard name");
        println!("name: {}", name);
        let hp = buf.read_u32::<LittleEndian>()?;
        let max_hp = buf.read_u32::<LittleEndian>()?;
        let acceptance = Acceptance::from_bytes(buf)?;
        let affinity = Affinity::from_bytes(buf)?;
        let status = StatusSet::from_bytes(buf)?;

        buf.read_exact(&mut usize_buf).expect("Failed to read wizard name size");
        let selected_spellbook = usize::from_le_bytes(usize_buf);
        let spellbooks = Vec::<SpellBook>::from_bytes(buf)?;

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

impl Inputable<Monster> for Monster {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        let name_len = buf.read_u8()?;
        let mut name = vec![0; name_len as usize];
        buf.read_exact(&mut name)?;
        let name = String::from_utf8(name).expect("Invalid utf8");
        let monster_type = MonsterType::from_u8(buf.read_u8()?);
        let hp = buf.read_u32::<LittleEndian>()?;
        let max_hp = buf.read_u32::<LittleEndian>()?;
        let affinity = Affinity::from_bytes(buf)?;
        let acceptance = Acceptance::from_bytes(buf)?;
        let status = StatusSet::from_bytes(buf)?;
        Ok(Self {
            id: None,
            name,
            monster_type,
            affinity,
            acceptance,
            hp,
            max_hp,
            status,
        })
    }
}

impl Inputable<Battle> for Battle {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        let active_allies = Vec::<usize>::from_bytes(buf)?;
        let active_enemies = Vec::<usize>::from_bytes(buf)?;
        let cast_wizard_spells = HashMap::<usize, Spell>::from_bytes(buf)?;
        let used_monster_abilities = HashMap::<usize, Spell>::from_bytes(buf)?;
        let past_ticks = Vec::<Vec<BattleEvent>>::from_bytes(buf)?;
        
        Ok(Battle {
            id: None,
            allies: Vec::new(),
            active_allies,
            cast_wizard_spells,
            enemies: Vec::new(),
            active_enemies,
            used_monster_abilities,
            past_ticks,
        })
    }
}

impl Inputable<BattleEvent> for BattleEvent {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        let event_type = buf.read_u8()?;
        Ok(match event_type {
            0 => BattleEvent::Wizard(BattleAtom::from_bytes(buf)?),
            1 => BattleEvent::Monster(BattleAtom::from_bytes(buf)?),
            2 => BattleEvent::Victory,
            3 => BattleEvent::Defeat,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid event type")),
        })
    }
}

impl Inputable<BattleAtom> for BattleAtom {
    fn from_bytes(buf: &mut std::io::Cursor<&[u8]>) -> Result<BattleAtom> {
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        let atom_type = buf.read_u8()?;
        match atom_type {
            0 => {
                buf.read_exact(&mut usize_buf)?;
                let caster = usize::from_le_bytes(usize_buf);
                let spell = Spell::from_bytes(buf)?;
                Ok(BattleAtom::CastSpell(caster, spell))
            }
            1 => {
                buf.read_exact(&mut usize_buf)?;
                let caster = usize::from_le_bytes(usize_buf);
                let spell = Spell::from_bytes(buf)?;
                Ok(BattleAtom::FizzleSpell(caster, spell))
            }
            2 => {
                buf.read_exact(&mut usize_buf)?;
                let caster = usize::from_le_bytes(usize_buf);
                let spell = Spell::from_bytes(buf)?;
                Ok(BattleAtom::SpellEnd(caster, spell))
            }
            3 => {
                buf.read_exact(&mut usize_buf)?;
                let caster = usize::from_le_bytes(usize_buf);
                let spell = Spell::from_bytes(buf)?;
                let effect_index = buf.read_u8()?;
                let progress = buf.read_u32::<LittleEndian>()?;
                Ok(BattleAtom::TickEffect(caster, spell, effect_index, progress))
            }
            4 => {
                buf.read_exact(&mut usize_buf)?;
                let damager = usize::from_le_bytes(usize_buf);
                buf.read_exact(&mut usize_buf)?;
                let damagee = usize::from_le_bytes(usize_buf);
                let damage = buf.read_u16::<LittleEndian>()?;
                let glyph = match buf.read_u8()? {
                    0 => Glyph::Fire,
                    1 => Glyph::Water,
                    2 => Glyph::Earth,
                    3 => Glyph::Air,
                    4 => Glyph::Void,
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid glyph")),
                };
                Ok(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, damage, glyph)))
            }
            5 => {
                buf.read_exact(&mut usize_buf)?;
                let healer = usize::from_le_bytes(usize_buf);
                buf.read_exact(&mut usize_buf)?;
                let healee = usize::from_le_bytes(usize_buf);
                let heal = buf.read_u16::<LittleEndian>()?;
                Ok(BattleAtom::Mutation(BattleMut::Heal(healer, healee, heal)))
            }
            6 => {
                buf.read_exact(&mut usize_buf)?;
                let statuser = usize::from_le_bytes(usize_buf);
                buf.read_exact(&mut usize_buf)?;
                let statusee = usize::from_le_bytes(usize_buf);
                let status = match buf.read_u8()? {
                    0 => Status::Burning,
                    1 => Status::Submerged,
                    2 => Status::Stunned,
                    3 => Status::Shocked,
                    4 => Status::Weakened,
                    5 => Status::Raging,
                    6 => Status::Hardened,
                    7 => Status::Fluid,
                    8 => Status::Flying,
                    9 => Status::Barrier(Glyph::Fire),
                    10 => Status::Barrier(Glyph::Water),
                    11 => Status::Barrier(Glyph::Earth),
                    12 => Status::Barrier(Glyph::Air),
                    13 => Status::Barrier(Glyph::Void),
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid status")),
                };
                let value = buf.read_u16::<LittleEndian>()?;
                let duration = buf.read_u16::<LittleEndian>()?;
                Ok(BattleAtom::Mutation(BattleMut::IncurStatus(statuser, statusee, status, value, duration)))
            }
            7 => {
                buf.read_exact(&mut usize_buf)?;
                let statuser = usize::from_le_bytes(usize_buf);
                buf.read_exact(&mut usize_buf)?;
                let statusee = usize::from_le_bytes(usize_buf);
                let status = match buf.read_u8()? {
                    0 => Status::Burning,
                    1 => Status::Submerged,
                    2 => Status::Stunned,
                    3 => Status::Shocked,
                    4 => Status::Weakened,
                    5 => Status::Raging,
                    6 => Status::Hardened,
                    7 => Status::Fluid,
                    8 => Status::Flying,
                    9 => Status::Barrier(Glyph::Fire),
                    10 => Status::Barrier(Glyph::Water),
                    11 => Status::Barrier(Glyph::Earth),
                    12 => Status::Barrier(Glyph::Air),
                    13 => Status::Barrier(Glyph::Void),
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid status")),
                };
                Ok(BattleAtom::Mutation(BattleMut::LoseStatus(statuser, statusee, status)))
            }
            8 => {
                buf.read_exact(&mut usize_buf)?;
                let killer = usize::from_le_bytes(usize_buf);
                buf.read_exact(&mut usize_buf)?;
                let killee = usize::from_le_bytes(usize_buf);
                Ok(BattleAtom::Kill(killer, killee))
            }
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid atom type")),
        }
    }
}
impl Inputable<SpellBook> for SpellBook {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        let glyphs = Affinity::from_bytes(buf)?;
        let style = Acceptance::from_bytes(buf)?;
        let mut spells = Vec::new();
        buf.read_exact(&mut usize_buf)?;
        let number_of_spells = usize::from_le_bytes(usize_buf);
        for _ in 0..number_of_spells {
            spells.push(Spell::from_bytes(buf)?);
        }
        Ok(SpellBook {
            id: None,
            spells,
            glyphs,
            style,
        })
    }
}

impl Inputable<Party> for Party {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Party> {
        
        let mut u8_16_buf = [0u8; 16];
        buf.read_exact(&mut u8_16_buf)?;
        let uuid = Uuid::from_slice(&u8_16_buf).unwrap();
        Ok(Party {
            id: None,
            uuid,
            members: Vec::new()
        })
    }
}

impl Inputable<Spell> for Spell {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        let glyph = Glyph::from_u8(buf.read_u8()?);
        let glyph_value = buf.read_u16::<LittleEndian>()?;
        let style = Style::from_u8(buf.read_u8()?);
        let style_value = buf.read_u16::<LittleEndian>()?;
        println!("{:?} {} {:?} {}", glyph, glyph_value, style, style_value);

        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf)?;
        let spell_index = usize::from_le_bytes(usize_buf);
        let ability = Ability::from_bytes(buf)?;
        Ok(Self {
            name: spells::NAME_BY_ID[spell_index],
            glyph: (glyph, glyph_value),
            style: (style, style_value),
            ability,
        })
    }
}

impl Inputable<Ability> for Ability {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        let priority = PriorityTypes::from_3u8(buf.read_u8()?, buf.read_u8()?, buf.read_u8()?)?;
        let target = TargetType::from_2u8(buf.read_u8()?, buf.read_u8()?)?;
        let effects = EffectProgression::from_bytes(buf)?;
        Ok(Self {
            priority,
            target,
            effects,
        })
    }
}

impl Inputable<EffectProgression> for EffectProgression {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        let kind = buf.read_u8()?;
        match kind {
            0 => Ok(Self::None),
            1 => Ok(Self::Single(Effect::from_bytes(buf)?)),
            2 => Ok(Self::Duo(Effect::from_bytes(buf)?, Effect::from_bytes(buf)?)),
            3 => Ok(Self::Trio(
                Effect::from_bytes(buf)?,
                Effect::from_bytes(buf)?,
                Effect::from_bytes(buf)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid effect progression kind",
            )),
        }
    }
}

impl Effect {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            value: buf.read_u16::<LittleEndian>()?,
            duration: EffectDuration::from_bytes(buf)?,
            application: EffectApplication::from_bytes(buf)?,
        })
    }
}

impl Inputable<EffectDuration> for EffectDuration {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        let kind = buf.read_u8()?;
        match kind {
            0 => Ok(Self::OverTime(buf.read_u16::<LittleEndian>()?)),
            1 => Ok(Self::Growth(
                buf.read_u16::<LittleEndian>()?,
                buf.read_u16::<LittleEndian>()?,
            )),
            2 => Ok(Self::AfterXTime(buf.read_u16::<LittleEndian>()?)),
            3 => Ok(Self::Instant),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid effect duration kind",
            )),
        }
    }
}

impl Inputable<EffectApplication> for EffectApplication {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Self> {
        let kind = buf.read_u8()?;
        match kind {
            0 => Ok(Self::Damage),
            1 => Ok(Self::Heal),
            2 => Ok(Self::Status(
                Status::from_u8(buf.read_u8()?),
                buf.read_u16::<LittleEndian>()?,
            )),
            3 => Ok(Self::RemoveStatus(Status::from_u8(buf.read_u8()?))),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid effect application kind",
            )),
        }
    }
}

impl Inputable<StatusSet> for StatusSet {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<StatusSet> {
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

impl Status {
    pub fn from_u8(id: u8) -> Self {
        match id {
            0 => Status::Barrier(Glyph::Fire),
            1 => Status::Barrier(Glyph::Water),
            2 => Status::Barrier(Glyph::Earth),
            3 => Status::Barrier(Glyph::Air),
            4 => Status::Barrier(Glyph::Void),
            5 => Status::Burning,
            6 => Status::Stunned,
            7 => Status::Submerged,
            8 => Status::Shocked,
            9 => Status::Weakened,
            10 => Status::Raging,
            11 => Status::Hardened,
            12 => Status::Fluid,
            13 => Status::Flying,
            _ => panic!("Invalid status byte: {}", id),
        }
    }
}

impl Inputable<Affinity> for Affinity {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Affinity> {
        Ok(Affinity { 
            fire: buf.read_u32::<LittleEndian>()?,
            air: buf.read_u32::<LittleEndian>()?,
            earth: buf.read_u32::<LittleEndian>()?,
            water: buf.read_u32::<LittleEndian>()?,
            void: buf.read_u32::<LittleEndian>()?,
        })
    }
}

impl Inputable<Acceptance> for Acceptance {
    fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<Acceptance> {
        let mut result = Acceptance {
            elder: buf.read_u32::<LittleEndian>()?,
            eldrich: buf.read_u32::<LittleEndian>()?,
            ancient: buf.read_u32::<LittleEndian>()?,
            arcane: buf.read_u32::<LittleEndian>()?,
            void: buf.read_u32::<LittleEndian>()?,
            highest: super::e::Style::Void,
        };
        result.reevaluate();
        Ok(result)
    }
}




