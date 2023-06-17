use std::io::Result;
use std::collections::HashMap;
use super::{e::{wiz::{Wizard, Affinity, Acceptance,}, status::{Status, StatusSet}, mon::{Monster, MonsterType}, party::Party, Glyph, Style, spell_book::SpellBook, spell::{Spell, PriorityType, PriorityTypes, spells, EffectProgression, Effect, Ability, TargetType, EffectDuration, EffectApplication}}};
use super::super::q::battle::{Battle, BattleEvent, BattleAtom, BattleMut};

use byteorder::{LittleEndian, WriteBytesExt};
use generational_arena::{Index, Arena};


pub(in super) trait Outputable {
    fn as_bytes(&self) -> Result<Vec<u8>>;
}

impl<T> Outputable for Vec<T> where T: Outputable + std::fmt::Debug {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        output.extend_from_slice(&self.len().to_le_bytes());
        for item in self.iter() {
            output.extend(item.as_bytes()?);
        }
        Ok(output)
    }
}

impl<T> Outputable for Arena<T> where T: Outputable {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        output.extend_from_slice(&self.len().to_le_bytes());
        for item in self.iter() {
            let item_as_bytes = item.1.as_bytes()?;
            output.extend_from_slice(&item_as_bytes.len().to_le_bytes());
            output.extend(item_as_bytes);
        }
        Ok(output)
    }
}

impl<T,U> Outputable for HashMap<T,U> where T: Outputable + std::fmt::Debug, U: Outputable + std::fmt::Debug {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        output.extend_from_slice(&self.len().to_le_bytes());
        for (key, value) in self.iter() {
            output.extend(key.as_bytes()?);
            output.extend(value.as_bytes()?);
        }
        Ok(output)
    }
}

impl Outputable for &str {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let as_bytes = str::as_bytes(self);
        let mut output = Vec::with_capacity(std::mem::size_of::<usize>() + as_bytes.len());
        output.extend_from_slice(&self.len().to_le_bytes());
        output.extend_from_slice(as_bytes);
        Ok(output)
    }
}

impl Outputable for String {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let as_bytes = String::as_bytes(self);
        let mut output = Vec::with_capacity(std::mem::size_of::<usize>() + as_bytes.len());
        output.extend_from_slice(&self.len().to_le_bytes());
        output.extend_from_slice(as_bytes);
        Ok(output)
    }
}

impl Outputable for usize {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(std::mem::size_of::<usize>());
        output.extend_from_slice(&self.to_le_bytes());
        Ok(output)
    }
}

impl Outputable for Index {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(std::mem::size_of::<usize>());
        output.extend_from_slice(&self.into_raw_parts().0.to_le_bytes());
        Ok(output)
    }
}

impl Outputable for Wizard {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let name_as_bytes = <String as Outputable>::as_bytes(&self.name)?;
        let acceptance_as_bytes = self.acceptance.as_bytes()?;
        let affinity_as_bytes = self.affinity.as_bytes()?;
        let status_as_bytes = self.status.as_bytes()?;
        let spellbooks_as_bytes = self.spellbooks.as_bytes()?;
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
            spellbooks_as_bytes.len() // spellbooks
        );
        output.extend(name_as_bytes);
        output.write_u32::<LittleEndian>(self.hp)?;
        output.write_u32::<LittleEndian>(self.max_hp)?;
        output.extend(acceptance_as_bytes);
        output.extend(affinity_as_bytes);
        output.extend(status_as_bytes);
        output.extend_from_slice(&self.selected_spellbook.to_le_bytes());
        output.extend(spellbooks_as_bytes);
        Ok(output)
    }
}

impl Outputable for Monster {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        
        let name_as_bytes = <String as Outputable>::as_bytes(&self.name)?;
        let affinity_as_bytes = self.affinity.as_bytes()?;
        let acceptance_as_bytes = self.acceptance.as_bytes()?;
        let status_as_bytes = self.status.as_bytes()?;
        let mut output = Vec::with_capacity(
                std::mem::size_of::<usize>()
                + name_as_bytes.len()
                + std::mem::size_of::<u8>()
                + std::mem::size_of::<u32>()
                + std::mem::size_of::<u32>()
                + affinity_as_bytes.len()
                + acceptance_as_bytes.len()
                + status_as_bytes.len());
        output.extend(name_as_bytes);
        output.write_u8(self.monster_type.as_u8())?;
        output.write_u32::<LittleEndian>(self.hp)?;
        output.write_u32::<LittleEndian>(self.max_hp)?;
        output.extend(self.affinity.as_bytes()?);
        output.extend(self.acceptance.as_bytes()?);
        output.extend(self.status.as_bytes()?);
        Ok(output)
    }
}

impl Outputable for Battle {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let allies_as_bytes = self.active_allies.as_bytes()?;
        let enemies_as_bytes = self.active_enemies.as_bytes()?;
        let cast_wizard_spells_as_bytes = self.cast_wizard_spells.as_bytes()?;
        let used_monster_abilities_as_bytes = self.used_monster_abilities.as_bytes()?;
        let ticks = self.past_ticks.as_bytes()?;
        let mut output = Vec::with_capacity(
            allies_as_bytes.len() + enemies_as_bytes.len() + cast_wizard_spells_as_bytes.len() + used_monster_abilities_as_bytes.len() + ticks.len());
        output.extend(allies_as_bytes);
        output.extend(enemies_as_bytes);
        output.extend(cast_wizard_spells_as_bytes);
        output.extend(used_monster_abilities_as_bytes);
        output.extend(ticks);
        Ok(output)
    }
}

impl Outputable for BattleEvent {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        match self {
            BattleEvent::Wizard(atom) | BattleEvent::Monster(atom) => {
                let atom_as_bytes = atom.as_bytes()?;
                let mut output = Vec::with_capacity(std::mem::size_of::<u8>() + atom_as_bytes.len());
                output.push(match self {
                    BattleEvent::Wizard(_) => 0,
                    BattleEvent::Monster(_) => 1,
                    _ => 100,
                });
                output.extend(atom_as_bytes);
                Ok(output)
            }
            BattleEvent::Victory => { Ok(vec![2]) },
            BattleEvent::Defeat => { Ok(vec![3]) },
        }
    }
}

impl Outputable for BattleAtom {
    fn as_bytes(&self) -> Result<Vec<u8>>{
        
        let mut output = Vec::new();
        match self {
            BattleAtom::CastSpell(caster, spell) => {
                output.write_u8(0)?;
                output.extend_from_slice(&caster.to_le_bytes());
                output.extend(spell.as_bytes()?);
            }
            BattleAtom::FizzleSpell(caster, spell) => {
                output.write_u8(1)?;
                output.extend_from_slice(&caster.to_le_bytes());
                output.extend(spell.as_bytes()?);
            }
            BattleAtom::SpellEnd(caster, spell) => {
                output.write_u8(2)?;
                output.extend_from_slice(&caster.to_le_bytes());
                output.extend(spell.as_bytes()?);
            }
            BattleAtom::TickEffect(caster, spell, effect_index, progress) => {
                output.write_u8(3)?;
                output.extend_from_slice(&caster.to_le_bytes());
                output.extend(spell.as_bytes()?);
                output.push(*effect_index);
                output.write_u32::<LittleEndian>(*progress)?;
            }
            BattleAtom::Mutation(battle_mut) => {
                match battle_mut {
                    BattleMut::Damage(damager, damagee, damage, glyph) => {
                        output.write_u8(4)?;
                        output.extend_from_slice(&damager.to_le_bytes());
                        output.extend_from_slice(&damagee.to_le_bytes());
                        output.write_u16::<LittleEndian>(*damage)?;
                        output.push(match glyph {
                            Glyph::Fire => 0,
                            Glyph::Water => 1,
                            Glyph::Earth => 2,
                            Glyph::Air => 3,
                            Glyph::Void => 4,
                        });
                    }
                    BattleMut::Heal(healer, healee, heal) => {
                        output.write_u8(5)?;
                        output.extend_from_slice(&healer.to_le_bytes());
                        output.extend_from_slice(&healee.to_le_bytes());
                        output.write_u16::<LittleEndian>(*heal)?;
                    }
                    BattleMut::IncurStatus(statuser, statusee, status, value, duration) => {
                        output.write_u8(6)?;
                        output.extend_from_slice(&statuser.to_le_bytes());
                        output.extend_from_slice(&statusee.to_le_bytes());
                        output.push(match status {
                            Status::Burning => 0,
                            Status::Submerged => 1,
                            Status::Stunned => 2,
                            Status::Shocked => 3,
                            Status::Weakened => 4,
                            Status::Raging => 5,
                            Status::Hardened => 6,
                            Status::Fluid => 7,
                            Status::Flying => 8,
                            Status::Barrier(glyph) => match glyph {
                                Glyph::Fire => 9,
                                Glyph::Water => 10,
                                Glyph::Earth => 11,
                                Glyph::Air => 12,
                                Glyph::Void => 13,
                            },
                        });
                        output.write_u16::<LittleEndian>(*value)?;
                        output.write_u16::<LittleEndian>(*duration)?;
                    }
                    BattleMut::LoseStatus(statuser, statusee, status) => {
                        output.write_u8(7)?;
                        output.extend_from_slice(&statuser.to_le_bytes());
                        output.extend_from_slice(&statusee.to_le_bytes());
                        output.push(match status {
                            Status::Burning => 0,
                            Status::Submerged => 1,
                            Status::Stunned => 2,
                            Status::Shocked => 3,
                            Status::Weakened => 4,
                            Status::Raging => 5,
                            Status::Hardened => 6,
                            Status::Fluid => 7,
                            Status::Flying => 8,
                            Status::Barrier(glyph) => match glyph {
                                Glyph::Fire => 9,
                                Glyph::Water => 10,
                                Glyph::Earth => 11,
                                Glyph::Air => 12,
                                Glyph::Void => 13,
                            },
                        });
                    }
                }
            }
            BattleAtom::Kill(killer, killee) => {
                output.write_u8(8)?;
                output.extend_from_slice(&killer.to_le_bytes());
                output.extend_from_slice(&killee.to_le_bytes());
            }
        }
        Ok(output)
    }
}


impl Outputable for Party {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.uuid.as_bytes().to_vec())
    }
}

impl Outputable for Glyph {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(vec![self.as_u8()])
    }
}

impl Outputable for Style {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(vec![self.as_u8()])
    }
}

impl Outputable for MonsterType {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(vec![self.as_u8()])
    }
}

impl Outputable for Affinity {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(std::mem::size_of::<u32>() * 5);
        output.write_u32::<LittleEndian>(self.fire).unwrap();
        output.write_u32::<LittleEndian>(self.air).unwrap();
        output.write_u32::<LittleEndian>(self.earth).unwrap();
        output.write_u32::<LittleEndian>(self.water).unwrap();
        output.write_u32::<LittleEndian>(self.void).unwrap();
        Ok(output)
    }
}

impl Outputable for Acceptance {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(std::mem::size_of::<u32>() * 5);
        output.write_u32::<LittleEndian>(self.elder).unwrap();
        output.write_u32::<LittleEndian>(self.eldrich).unwrap();
        output.write_u32::<LittleEndian>(self.ancient).unwrap();
        output.write_u32::<LittleEndian>(self.arcane).unwrap();
        output.write_u32::<LittleEndian>(self.void).unwrap();
        Ok(output)
    }
}

impl Outputable for Status {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(std::mem::size_of::<u32>());
        match self {
            Status::Burning => { output.write_u32::<LittleEndian>(0)?; },
            Status::Submerged => { output.write_u32::<LittleEndian>(1)?; },
            Status::Stunned => { output.write_u32::<LittleEndian>(2)?; },
            Status::Shocked => { output.write_u32::<LittleEndian>(3)?; },
            Status::Weakened => { output.write_u32::<LittleEndian>(4)?; },
            Status::Raging => { output.write_u32::<LittleEndian>(5)?; },
            Status::Hardened => { output.write_u32::<LittleEndian>(6)?; },
            Status::Fluid => { output.write_u32::<LittleEndian>(7)?; },
            Status::Flying => { output.write_u32::<LittleEndian>(8)?; },
            Status::Barrier(Glyph::Fire) => { output.write_u32::<LittleEndian>(9)?; },
            Status::Barrier(Glyph::Water) => { output.write_u32::<LittleEndian>(10)?; },
            Status::Barrier(Glyph::Earth) => { output.write_u32::<LittleEndian>(11)?; },
            Status::Barrier(Glyph::Air) => { output.write_u32::<LittleEndian>(12)?; },
            Status::Barrier(Glyph::Void) => { output.write_u32::<LittleEndian>(13)?; },
        }
        Ok(output)
    }
}

impl Outputable for BattleMut {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        match self {
            BattleMut::Damage(damager, damagee, damage, glyph) => {
                output.write_u8(0)?;
                output.extend_from_slice(&damager.to_le_bytes());
                output.extend_from_slice(&damagee.to_le_bytes());
                output.write_u16::<LittleEndian>(*damage)?;
                output.push(glyph.as_u8());
            }
            BattleMut::Heal(healer, healee, heal) => {
                output.write_u8(1)?;
                output.extend_from_slice(&healer.to_le_bytes());
                output.extend_from_slice(&healee.to_le_bytes());
                output.write_u16::<LittleEndian>(*heal)?;
            }
            BattleMut::IncurStatus(statuser, statusee, status, value, duration) => {
                output.write_u8(2)?;
                output.extend_from_slice(&statuser.to_le_bytes());
                output.extend_from_slice(&statusee.to_le_bytes());
                output.push(match status {
                    Status::Burning => 0,
                    Status::Submerged => 1,
                    Status::Stunned => 2,
                    Status::Shocked => 3,
                    Status::Weakened => 4,
                    Status::Raging => 5,
                    Status::Hardened => 6,
                    Status::Fluid => 7,
                    Status::Flying => 8,
                    Status::Barrier(glyph) => 8 + glyph.as_u8(),
                });
                output.write_u16::<LittleEndian>(*value)?;
                output.write_u16::<LittleEndian>(*duration)?;
            }
            BattleMut::LoseStatus(statuser, statusee, status) => {
                output.write_u8(3)?;
                output.extend_from_slice(&statuser.to_le_bytes());
                output.extend_from_slice(&statusee.to_le_bytes());
                output.extend(status.as_bytes()?);
            }
        }
        Ok(output)
    }
}

impl Outputable for StatusSet {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(std::mem::size_of::<u16>()*14*2);
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
}

impl Outputable for SpellBook {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new(); 
        output.extend(self.glyphs.as_bytes()?);
        output.extend(self.style.as_bytes()?);
        output.extend_from_slice(&self.spells.len().to_le_bytes());
        for spell in &self.spells {
            output.extend(spell.as_bytes()?);
        }
        Ok(output)
    }
}

impl Outputable for Spell {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        output.write_u8(self.glyph.0.as_u8())?;
        output.write_u16::<LittleEndian>(self.glyph.1)?;
        output.write_u8(self.style.0.as_u8())?;
        output.write_u16::<LittleEndian>(self.style.1)?;
        output.extend(spells::ID_BY_NAME[self.name].as_bytes()?);
        output.extend(self.ability.as_bytes()?);
        Ok(output)
    }
}

impl Outputable for PriorityTypes {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(match self {
            PriorityTypes::Single(priority) => vec![0, priority.as_u8(), 0],
            PriorityTypes::Or(priority1, priority2) => vec![1, priority1.as_u8(), priority2.as_u8()],
            PriorityTypes::And(priority1, priority2) => vec![2, priority1.as_u8(), priority2.as_u8()],
        })
    }
}
impl PriorityType {
    fn as_u8(&self) -> u8 {
        match self {
            PriorityType::Squishy => 0,
            PriorityType::Tanky => 1,
            PriorityType::LowHealth => 2,
            PriorityType::HighHealth => 3,
            PriorityType::HasStatus(status) => 4 + status.as_u8(),
            PriorityType::NoStatus(status) => 18 + status.as_u8(),
        }
    }
}

impl Outputable for EffectProgression {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        match self {
            EffectProgression::None => {
                Ok(vec![0])
            }
            EffectProgression::Single(effect) => {
                let effect = effect.as_bytes()?;
                let mut output = Vec::with_capacity(1 + effect.len());
                output.write_u8(1)?;
                output.extend(effect);
                Ok(output)
            }
            EffectProgression::Duo(effect1, effect2) => {
                let effect1 = effect1.as_bytes()?;
                let effect2 = effect2.as_bytes()?;
                let mut output = Vec::with_capacity(1 + effect1.len() + effect2.len());
                output.write_u8(2)?;
                output.extend(effect1);
                output.extend(effect2);
                Ok(output)
            }
            EffectProgression::Trio(effect1, effect2, effect3) => {
                let effect1 = effect1.as_bytes()?;
                let effect2 = effect2.as_bytes()?;
                let effect3 = effect3.as_bytes()?;
                let mut output = Vec::with_capacity(1 + effect1.len() + effect2.len() + effect3.len());
                output.write_u8(3)?;
                output.extend(effect1);
                output.extend(effect2);
                output.extend(effect3);
                Ok(output)
            }
        }
    }
}

impl Outputable for Effect {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let duration = self.duration.as_bytes()?;
        let application = self.application.as_bytes()?;
        let mut output = Vec::with_capacity(2 + duration.len() + application.len());
        output.write_u16::<LittleEndian>(self.value)?;
        output.extend(duration);
        output.extend(application);
        Ok(output)
    }
}

impl Outputable for Ability {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let priority = self.priority.as_bytes()?;
        let target = self.target.as_2u8();
        let effects = self.effects.as_bytes()?;

        let mut output = Vec::with_capacity(priority.len() + target.len() + effects.len());
        output.extend(priority);
        output.extend(target);
        output.extend(effects);
        Ok(output)
    }
}

impl EffectDuration {
    
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        match self {
            EffectDuration::OverTime(duration) => {
                output.write_u8(0)?;
                output.write_u16::<LittleEndian>(*duration)?;
            },
            EffectDuration::Growth(duration, value) => {
                output.write_u8(1)?;
                output.write_u16::<LittleEndian>(*duration)?;
                output.write_u16::<LittleEndian>(*value)?;
            },
            EffectDuration::AfterXTime(duration) => {
                output.write_u8(2)?;
                output.write_u16::<LittleEndian>(*duration)?;
            },
            EffectDuration::Instant => {
                output.write_u8(3)?;
            },
        }
        Ok(output)
    }
}

impl Outputable for EffectApplication {
    fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        match self {
            EffectApplication::Damage => {
                output.write_u8(0)?;
            },
            EffectApplication::Heal => {
                output.write_u8(1)?;
            },
            EffectApplication::Status(status, duration) => {
                output.write_u8(2)?;
                output.write_u8(status.as_u8())?;
                output.write_u16::<LittleEndian>(*duration)?;
            },
            EffectApplication::RemoveStatus(status) => {
                output.write_u8(3)?;
                output.write_u8(status.as_u8())?;
            },
        }
        Ok(output)
    }
}


impl TargetType {
    pub fn as_2u8(&self) -> [u8;2] {
        match self {
            TargetType::MeAlone => [0,0],
            TargetType::Ally(index) => [1,*index],
            TargetType::Enemy(index) => [2,*index],
        }
    }
}
impl Status {
    pub fn as_u8(&self) -> u8 {
        match self {
            Status::Barrier(Glyph::Fire) => 0,
            Status::Barrier(Glyph::Water) => 1,
            Status::Barrier(Glyph::Earth) => 2,
            Status::Barrier(Glyph::Air) => 3,
            Status::Barrier(Glyph::Void) => 4,
            Status::Burning => 5,
            Status::Stunned => 6,
            Status::Submerged => 7,
            Status::Shocked => 8,
            Status::Weakened => 9,
            Status::Raging => 10,
            Status::Hardened => 11,
            Status::Fluid => 12,
            Status::Flying => 13,
        }
    }
}
