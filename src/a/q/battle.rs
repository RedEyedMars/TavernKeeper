use crate::a::e::{spell::{Spell, Effect, Status}, wiz::Wizard, mon::Monster};


pub enum BattleEvent {
    Victory,
    Defeat,
    Death(Wizard),
    Kill(Monster),
    Damage(Wizard, u32),
    DamageMonster(Monster, u32),
    Heal(Wizard, u32),
    HealMonster(Monster, u32),
    IncurStatus(Wizard, Status),
    IncurStatusMonster(Monster, Status),
    LooseStatus(Wizard, Status),
    LooseStatusMonster(Monster, Status),
    CastSpell(Wizard, u32),
    UseAbility(Monster, u32),
    TickEffect(Effect),
    TickEffectMonster(Effect),
}

use std::collections::HashMap;
use generational_arena::Index;
type Tick = Vec<BattleEvent>;
pub struct Battle<const W: usize, const M: usize> {
    pub allies: [Index; W], // index points to Arena<Wizard>[Index]
    pub cast_wizard_spells: HashMap<Index,Spell>, // index points to Arena<Wizard>[Index]
    pub enemies: [Index; M], // index points to Arena<Monster>[Index]
    pub used_monster_abilities: HashMap<Index,Spell>, // index points to Arena<Monster>[Index]
    pub past_ticks: Vec<Tick>,
    pub upcoming_tick: Tick,
}