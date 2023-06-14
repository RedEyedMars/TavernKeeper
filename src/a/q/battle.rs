use crate::a::e::{spell::{Spell, Effect, Status, PriorityType, TargetType, EffectApplication, PriorityTypes}, wiz::{Wizard, Affinity, StatusSet}, mon::Monster, Glyph, col::{Colosseum, ColosseumArena, Idable}};
use std::{collections::HashMap, cmp::Ordering, usize};
use generational_arena::Index;
pub type Tick = Vec<BattleEvent>;
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BattleAtom {
    Kill(usize, usize), // Killer, Killed
    Mutation(BattleMut),
    CastSpell(usize, Spell),
    TickEffect(usize, Spell, u8, u32), // caster, spell,index of effect, progress
    SpellEnd(usize, Spell),
    FizzleSpell(usize, Spell),
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BattleMut {
    Damage(usize, usize, u16, Glyph),
    Heal(usize, usize, u16),
    IncurStatus(usize, usize, Status, u16, u16),
    LoseStatus(usize, usize, Status),
}
impl BattleMut {
    pub fn affectee(&self) -> usize {
        match self {
            BattleMut::Damage(_damager, damagee, _damage, _glyph) => *damagee,
            BattleMut::Heal(_healer, healee, _heal) => *healee,
            BattleMut::IncurStatus(_statuser, statusee, _status, _value, _duration) => *statusee,
            BattleMut::LoseStatus(_statuser, statusee, _status) => *statusee,
        }
    }
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BattleEvent {
    Monster(BattleAtom),
    Wizard(BattleAtom),
    Victory,
    Defeat,
}

#[derive(Clone, Debug)]
pub struct Battle {
    pub id: Option<Index>,
    pub allies: Vec<Index>, // index points to Arena<Wizard>[Index]
    pub active_allies: Vec<usize>,
    pub cast_wizard_spells: HashMap<usize,Spell>, // index points to Arena<Wizard>[Index]
    pub enemies: Vec<Index>, // index points to Arena<Monster>[Index]
    pub active_enemies: Vec<usize>,
    pub used_monster_abilities: HashMap<usize,Spell>, // index points to Arena<Monster>[Index]
    pub past_ticks: Vec<Tick>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TargetHealth {
    NoHealth, LowHealth, MediumHealth, HighHealth, FullHealth
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TargetSqiushy {
    Squishy, MidRange, Tanky
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Target {
    pub index: usize,
    pub health: TargetHealth,
    pub squishy: TargetSqiushy,
    pub statuses: StatusSet,
    pub augment: Affinity,
    pub augment_cast: Affinity,
    pub resist: Affinity,
}

impl Battle {
    pub fn new(allies: Vec<Index>, enemies: Vec<Index>) -> Self {
        let allies_len = allies.len();
        let enemies_len = enemies.len();
        Self {
            id: None,
            allies: allies,
            active_allies: (0..allies_len).into_iter().collect::<Vec<usize>>(),
            cast_wizard_spells: HashMap::new(),
            enemies: enemies,
            active_enemies: (0..enemies_len).into_iter().collect::<Vec<usize>>(),
            used_monster_abilities: HashMap::new(),
            past_ticks: Vec::new(),
        }
    }

    pub fn push_ally(&mut self, ally: Index) {
        self.allies.push(ally);
    }
    pub fn push_enemy(&mut self, enemy: Index) {
        self.enemies.push(enemy);
    }

    pub fn as_text(&self) -> String {
        format!("{:#?}", self.past_ticks)
    }
   
    pub fn run(mut self, col: &mut Colosseum) {
        let mut tick = Vec::new();
        loop {
            tick = self.tick(&mut tick, col);
            if tick.iter().any(|event| event == &BattleEvent::Victory || event == &BattleEvent::Defeat) {
                break;
            }
        }
    }

    pub fn tick(&mut self, currect_tick: &mut Tick, col: &mut Colosseum) -> Tick {
        let mut tick = Vec::new();

        {
            let mut killed_allies = Vec::new();
            let mut killed_enemies = Vec::new();
            for event in currect_tick.iter() {
                match event {
                    BattleEvent::Wizard(BattleAtom::Kill(_killer, killed)) => killed_allies.push(killed),
                    BattleEvent::Monster(BattleAtom::Kill(_killer, killed)) => killed_enemies.push(killed),
                    _ => {}
                }
            }
            
            if killed_allies.len() > 0 {
                self.active_allies = self.active_allies.iter()
                    .filter(|index| !killed_allies.contains(index))
                    .map(|index| index.clone())
                    .collect::<Vec<usize>>();
            }
            if killed_enemies.len() > 0 {
                self.active_enemies = self.active_enemies.iter()
                    .filter(|index| !killed_enemies.contains(index))
                    .map(|index| index.clone())
                    .collect::<Vec<usize>>();
            }

            if self.active_allies.len() == 0 {
                self.past_ticks.push(currect_tick.clone());
                tick.push(BattleEvent::Defeat);
                self.past_ticks.push(tick.clone());
                return tick;
            }

            if self.active_enemies.len() == 0 {
                self.past_ticks.push(currect_tick.clone());
                tick.push(BattleEvent::Victory);
                self.past_ticks.push(tick.clone());
                return tick;
            }
        }

        
        let mut wizards_as_targets = self.active_allies.iter()
            .map(|index| col.as_target::<Wizard>(self.allies[*index], *index))
            .collect::<Vec<Target>>();
        let mut monsters_as_targets = self.active_enemies.iter()
                .map(|index| col.as_target::<Monster>(self.allies[*index], *index))
                .collect::<Vec<Target>>();
        {
            for event in currect_tick.iter() {
                match event {
                    BattleEvent::Wizard(BattleAtom::CastSpell(index, spell)) => {
                        self.cast_wizard_spells.insert(*index, spell.clone());
                        tick.push(BattleEvent::Wizard(BattleAtom::TickEffect(index.clone(), spell.clone(), 1, 0)));
                    }
                    BattleEvent::Monster(BattleAtom::CastSpell(index, spell)) => {
                        self.used_monster_abilities.insert(*index, spell.clone());
                        tick.push(BattleEvent::Monster(BattleAtom::TickEffect(index.clone(), spell.clone(), 1, 0)));
                    }
                    BattleEvent::Wizard(BattleAtom::SpellEnd(index, _spell)) => {
                        self.cast_wizard_spells.remove(index);
                    }
                    BattleEvent::Monster(BattleAtom::SpellEnd(index, _spell)) => {
                        self.used_monster_abilities.remove(index);
                    }
                    BattleEvent::Wizard(BattleAtom::FizzleSpell(index, _spell)) => {
                        self.cast_wizard_spells.remove(index);
                    }
                    BattleEvent::Monster(BattleAtom::FizzleSpell(index, _spell)) => {
                        self.used_monster_abilities.remove(index);
                    }
                    _ => {}
                }
            }
            
            for wizard in self.active_allies.iter() {
                if !self.cast_wizard_spells.contains_key(wizard) {
                    let wiz: &Wizard = col.get(self.allies[*wizard]);
                    let spells = wiz.get_spells();
                    let spell = Self::pick_spell(spells, &wizards_as_targets, &monsters_as_targets);
                    tick.push(BattleEvent::Wizard(BattleAtom::CastSpell(wizard.clone(), spell)));
                }
            }
            for monster in self.active_allies.iter() {
                if !self.used_monster_abilities.contains_key(monster) {
                    let mon: &Monster = col.get(self.enemies[*monster]);
                    let spells = &mon.get_abilities();
                    let spell = Self::pick_spell(spells, &monsters_as_targets, &wizards_as_targets);
                    tick.push(BattleEvent::Monster(BattleAtom::CastSpell(monster.clone(), spell)));
                }
            }
        }
        let mut battle_ticks = Vec::new();
        {
            for event in currect_tick.iter() {
                match event {
                    BattleEvent::Wizard(BattleAtom::TickEffect(index, spell, effect_index, progress)) => {
                        let wizard: &Wizard = col.get(self.allies[*index]);
                        Self::tick_effect(&mut tick, wizard, &wizard.as_target(*index),  &spell, &effect_index, &progress, &mut battle_ticks, &mut wizards_as_targets, &mut monsters_as_targets);
                    }
                    BattleEvent::Monster(BattleAtom::TickEffect(index, spell, effect_index, progress)) => {
                        let monster: &Monster = col.get(self.enemies[*index]);
                        Self::tick_effect(&mut tick, monster, &monster.as_target(*index),  &spell, &effect_index, &progress, &mut battle_ticks, &mut wizards_as_targets, &mut monsters_as_targets);
                    }
                    _ => {}
                }
            }
        }

        for wizard_target in wizards_as_targets {
            let wizard: &mut Wizard = col.get_mut(self.allies[wizard_target.index]);
            tick = Self::tick_wiz_status(&wizard_target.index, &wizard.status, &mut battle_ticks, tick);
            wizard.status.tick_all();
        }

        for monster_target in monsters_as_targets {
            let monster: &mut Monster = col.get_mut(self.enemies[monster_target.index]);
            tick = Self::tick_mon_status(&monster_target.index, &monster.status, &mut battle_ticks, tick);
            monster.status.tick_all();
        }

        {
            for battle_tick in battle_ticks.into_iter() {
                currect_tick.push(battle_tick.clone());
                match battle_tick {
                    BattleEvent::Wizard(BattleAtom::Mutation(battle_mut)) => {
                        //BattleAtom::Damage(damager, damagee, damage, _)
                        let affectee: &mut Wizard = col.get_mut(self.allies[battle_mut.affectee()]);
                        Self::mutate(&battle_mut, affectee, &mut tick);
                    }
                    BattleEvent::Monster(BattleAtom::Mutation(battle_mut)) => {
                        //BattleAtom::Damage(damager, damagee, damage, _)
                        let affectee: &mut Wizard = col.get_mut(self.allies[battle_mut.affectee()]);
                        Self::mutate(&battle_mut, affectee, &mut tick);
                    }
                    _ => {}
                };
            }
        }
        self.past_ticks.push(currect_tick.clone());
        tick
    }

    fn tick_wiz_status(affectee: &usize, statuses: &StatusSet, battle: &mut Vec<BattleEvent>, tick: Tick) -> Tick {
        if let Some(s) = statuses.entry(&Status::Burning) {
            battle.push(BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Damage(*affectee, *affectee, s, Glyph::Fire))));
        }
        if let Some(s) = statuses.entry(&Status::Submerged) {
            battle.push(BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Damage(*affectee, *affectee, s, Glyph::Water))));
        }
        return tick.into_iter().map(|event| {
            match event {
                BattleEvent::Wizard(BattleAtom::CastSpell(caster, spell)) 
                | BattleEvent::Wizard(BattleAtom::TickEffect(caster, spell, _, _))
                    if statuses.has(&Status::Stunned) && caster == *affectee => BattleEvent::Wizard(BattleAtom::FizzleSpell(caster, spell)),
                BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Heal(healer, healee, heal))) if healer == *affectee || healee == *affectee => 
                    BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Heal(healer, healee, heal - statuses.value(&Status::Shocked)))),
                BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, damage, glyph))) if damagee == *affectee =>
                    BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, 
                        damage.saturating_sub(statuses.value(&Status::Barrier(glyph.clone())))
                            .saturating_sub(statuses.value(&Status::Hardened)),
                        glyph))),
                BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, damage, glyph))) if damager == *affectee =>
                    BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, 
                        damage.saturating_add(statuses.value(&Status::Raging))
                            .saturating_add(statuses.value(&Status::Fluid))
                            .saturating_add(statuses.value(&Status::Flying))
                            .saturating_sub(statuses.value(&Status::Weakened))
                            .saturating_sub(statuses.value(&Status::Hardened)),
                        glyph))),
                BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, damage, glyph))) if damagee == *affectee =>
                    BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, 
                        damage.saturating_add(statuses.value(&Status::Raging))
                            .saturating_sub(statuses.value(&Status::Flying)), 
                        glyph))),
                _ => event,
            }
        }
        ).collect::<Tick>();
        
    }

    fn tick_mon_status(affectee: &usize, statuses: &StatusSet, battle: &mut Vec<BattleEvent>, tick: Tick) -> Tick {
        if let Some(s) = statuses.entry(&Status::Burning) {
            battle.push(BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Damage(*affectee, *affectee, s, Glyph::Fire))));
        }
        if let Some(s) = statuses.entry(&Status::Submerged) {
            battle.push(BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Damage(*affectee, *affectee, s, Glyph::Water))));
        }
        return tick.into_iter().map(|event| {
            match event {
                BattleEvent::Monster(BattleAtom::CastSpell(caster, spell)) 
                | BattleEvent::Monster(BattleAtom::TickEffect(caster, spell, _, _))
                    if statuses.has(&Status::Stunned) && caster == *affectee => BattleEvent::Monster(BattleAtom::FizzleSpell(caster, spell)),
                
                BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Heal(healer, healee, heal))) if healer == *affectee || healee == *affectee => {
                    if let Some(s) = statuses.entry(&Status::Shocked) {
                        return BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Heal(healer, healee, heal - s)));
                    }
                    event
                }
                BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, damage, glyph))) if damagee == *affectee =>
                    BattleEvent::Monster(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, 
                        damage.saturating_sub(statuses.value(&Status::Barrier(glyph.clone())))
                            .saturating_sub(statuses.value(&Status::Hardened)), glyph))),
                BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, damage, glyph))) if damager == *affectee =>
                    BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Damage(damager, damagee,
                        (damage 
                            + statuses.value(&Status::Raging) 
                            + statuses.value(&Status::Fluid) 
                            + statuses.value(&Status::Flying))
                            .saturating_sub(statuses.value(&Status::Weakened))
                        , glyph))),
                BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, damage, glyph))) if damagee == *affectee =>
                    BattleEvent::Wizard(BattleAtom::Mutation(BattleMut::Damage(damager, damagee, 
                        (damage 
                            + statuses.value(&Status::Raging))
                            .saturating_sub(statuses.value(&Status::Flying))
                        , glyph))),
                _ => event,
            }
        }
        ).collect::<Tick>();
    }

    fn sort_spell_by_targets(spell: &Spell, targets: &mut Vec<Target>) {
        match spell.priority_types() {
            PriorityTypes::Single(priority_type) => targets.sort_by(Self::compare_spell_targets(&priority_type)),
            PriorityTypes::Or(p1, p2) => {
                let cmp1 = Self::compare_spell_targets(&p1);
                let cmp2 = Self::compare_spell_targets(&p2);
                targets.sort_by(|a, b| {
                    let o1 = cmp1(a,b);
                    let o2 = cmp2(a,b);
                    if o1 == Ordering::Less || o2 == Ordering::Less {
                        Ordering::Less
                    } else if o1 == Ordering::Equal && o2 == Ordering::Equal {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                });
            }
            PriorityTypes::And(p1, p2) => {
                targets.sort_by(Self::compare_spell_targets(&p1));
                targets.sort_by(Self::compare_spell_targets(&p2));
            }
        }
    }    

    fn pick_spell(spells: &Vec<Spell>, allies: &Vec<Target>, enemies: &Vec<Target>) -> Spell {
        let mut best_spell = spells.first().unwrap();
        let mut best_score = 0f32;
        for spell in spells.iter() {
            let mut score = 0f32;
            for priority_types in spell.priorities() {
                let len = priority_types.len() as f32;
                let (targets, num_of) = match spell.target() {
                    TargetType::Ally(num) => (allies, *num as f32),
                    TargetType::MeAlone => (allies, 1f32), // todo
                    TargetType::Enemy(num) => (enemies, *num as f32),
                };
                for priority_type in priority_types.iter() {
                    let mut times_scored = 0f32;
                    for target in targets.iter() {
                        if Self::target_scores_on_priority(target, priority_type) {
                            score += 1f32 / num_of / len;
                            times_scored += 1f32;
                            if num_of >= times_scored {
                                break;
                            }
                        }
                    }
                }
            }
            if score > best_score {
                best_score = score;
                best_spell = spell;
            }
        }
        best_spell.clone()
    }

    fn target_scores_on_priority(target: &Target, priority_type: &PriorityType) -> bool {
        match priority_type {
            PriorityType::LowHealth => target.health == TargetHealth::LowHealth,
            PriorityType::HighHealth => target.health == TargetHealth::HighHealth 
                                        || target.health == TargetHealth::FullHealth,
            PriorityType::Squishy => target.squishy == TargetSqiushy::Squishy,
            PriorityType::Tanky => target.squishy == TargetSqiushy::Tanky,
            PriorityType::HasStatus(status) => target.statuses.has(status),
            PriorityType::NoStatus(status) => !target.statuses.has(status),
        }
    }
    
    fn compare_spell_targets(priority: &PriorityType) -> fn(&Target, &Target) -> Ordering {
        match priority {
            PriorityType::LowHealth => |a: &Target, b: &Target| 
                match a.health { 
                    TargetHealth::LowHealth => match b.health { 
                        TargetHealth::LowHealth => Ordering::Equal,
                        _ => Ordering::Less 
                    }
                    _ => Ordering::Greater
                }, 
            PriorityType::HighHealth => |a: &Target, b: &Target| 
                match a.health {
                    TargetHealth::HighHealth | TargetHealth::FullHealth => match b.health {
                        TargetHealth::HighHealth | TargetHealth::FullHealth => Ordering::Equal,
                        _ => Ordering::Less
                    },
                    _ => Ordering::Greater
                },
            PriorityType::Squishy => |a: &Target, b: &Target| 
                match a.squishy {
                    TargetSqiushy::Squishy => match b.squishy {
                        TargetSqiushy::Squishy => Ordering::Equal,
                        _ => Ordering::Less
                    },
                    _ => Ordering::Greater
                },
            PriorityType::Tanky => |a: &Target, b: &Target|
                match a.squishy {
                    TargetSqiushy::Tanky => match b.squishy {
                        TargetSqiushy::Tanky => Ordering::Equal,
                        _ => Ordering::Less
                    },
                    _ => Ordering::Greater
                },
            PriorityType::HasStatus(status) => 
                match status {
                    Status::Burning => |a: &Target, b: &Target| {
                        if a.statuses.has(&Status::Burning) {
                            if b.statuses.has(&Status::Burning) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Submerged => |a: &Target, b: &Target| {
                        if a.statuses.has(&Status::Submerged) {
                            if b.statuses.has(&Status::Submerged) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Stunned => |a: &Target, b: &Target| {
                        if a.statuses.has(&Status::Stunned) {
                            if b.statuses.has(&Status::Stunned) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Shocked => |a: &Target, b: &Target| {
                        if a.statuses.has(&Status::Shocked) {
                            if b.statuses.has(&Status::Shocked) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Weakened => |a: &Target, b: &Target| {
                        if a.statuses.has(&Status::Weakened) {
                            if b.statuses.has(&Status::Weakened) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Raging => |a: &Target, b: &Target| {
                        if a.statuses.has(&Status::Raging) {
                            if b.statuses.has(&Status::Raging) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Hardened => |a: &Target, b: &Target| {
                        if a.statuses.has(&Status::Hardened) {
                            if b.statuses.has(&Status::Hardened) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Fluid => |a: &Target, b: &Target| {
                        if a.statuses.has(&Status::Fluid) {
                            if b.statuses.has(&Status::Fluid) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Flying => |a: &Target, b: &Target| {
                        if a.statuses.has(&Status::Flying) {
                            if b.statuses.has(&Status::Flying) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Barrier(glyph) => match glyph {
                        Glyph::Fire => |a: &Target, b: &Target| {
                            if a.statuses.has(&Status::Barrier(Glyph::Fire)) {
                                if b.statuses.has(&Status::Barrier(Glyph::Fire)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                        Glyph::Water => |a: &Target, b: &Target| {
                            if a.statuses.has(&Status::Barrier(Glyph::Water)) {
                                if b.statuses.has(&Status::Barrier(Glyph::Water)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                        Glyph::Earth => |a: &Target, b: &Target| {
                            if a.statuses.has(&Status::Barrier(Glyph::Earth)) {
                                if b.statuses.has(&Status::Barrier(Glyph::Earth)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                        Glyph::Air => |a: &Target, b: &Target| {
                            if a.statuses.has(&Status::Barrier(Glyph::Air)) {
                                if b.statuses.has(&Status::Barrier(Glyph::Air)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                        Glyph::Void => |a: &Target, b: &Target| {
                            if a.statuses.has(&Status::Barrier(Glyph::Void)) {
                                if b.statuses.has(&Status::Barrier(Glyph::Void)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                    },
                },
            PriorityType::NoStatus(status) => 
                match status {
                    Status::Burning => |a: &Target, b: &Target| {
                        if !a.statuses.has(&Status::Burning) {
                            if !b.statuses.has(&Status::Burning) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Submerged => |a: &Target, b: &Target| {
                        if !a.statuses.has(&Status::Submerged) {
                            if !b.statuses.has(&Status::Submerged) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Stunned => |a: &Target, b: &Target| {
                        if !a.statuses.has(&Status::Stunned) {
                            if !b.statuses.has(&Status::Stunned) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Shocked => |a: &Target, b: &Target| {
                        if !a.statuses.has(&Status::Shocked) {
                            if !b.statuses.has(&Status::Shocked) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Weakened => |a: &Target, b: &Target| {
                        if !a.statuses.has(&Status::Weakened) {
                            if !b.statuses.has(&Status::Weakened) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Raging => |a: &Target, b: &Target| {
                        if !a.statuses.has(&Status::Raging) {
                            if !b.statuses.has(&Status::Raging) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Hardened => |a: &Target, b: &Target| {
                        if !a.statuses.has(&Status::Hardened) {
                            if !b.statuses.has(&Status::Hardened) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Fluid => |a: &Target, b: &Target| {
                        if !a.statuses.has(&Status::Fluid) {
                            if !b.statuses.has(&Status::Fluid) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Flying => |a: &Target, b: &Target| {
                        if !a.statuses.has(&Status::Flying) {
                            if !b.statuses.has(&Status::Flying) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        } else {
                            Ordering::Greater
                        }
                    },
                    Status::Barrier(glyph) => match glyph {
                        Glyph::Fire => |a: &Target, b: &Target| {
                            if !a.statuses.has(&Status::Barrier(Glyph::Fire)) {
                                if !b.statuses.has(&Status::Barrier(Glyph::Fire)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                        Glyph::Water => |a: &Target, b: &Target| {
                            if !a.statuses.has(&Status::Barrier(Glyph::Water)) {
                                if !b.statuses.has(&Status::Barrier(Glyph::Water)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                        Glyph::Earth => |a: &Target, b: &Target| {
                            if !a.statuses.has(&Status::Barrier(Glyph::Earth)) {
                                if !b.statuses.has(&Status::Barrier(Glyph::Earth)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                        Glyph::Air => |a: &Target, b: &Target| {
                            if !a.statuses.has(&Status::Barrier(Glyph::Air)) {
                                if !b.statuses.has(&Status::Barrier(Glyph::Air)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                        Glyph::Void => |a: &Target, b: &Target| {
                            if !a.statuses.has(&Status::Barrier(Glyph::Void)) {
                                if !b.statuses.has(&Status::Barrier(Glyph::Void)) {
                                    Ordering::Equal
                                } else {
                                    Ordering::Less
                                }
                            } else {
                                Ordering::Greater
                            }
                        },
                    },
                },
        }
    }

    pub fn as_output(&self) -> std::io::Result<Vec<u8>> {
        let mut output = Vec::new();
        output.extend_from_slice(&self.active_allies.len().to_le_bytes());
        for ally in self.active_allies.iter() {
            output.extend_from_slice(&ally.to_le_bytes());
        }
        output.extend_from_slice(&self.active_enemies.len().to_le_bytes());
        for enemy in self.active_enemies.iter() {
            output.extend_from_slice(&enemy.to_le_bytes());
        }
        output.extend_from_slice(&self.cast_wizard_spells.len().to_le_bytes());
        for (key, spell) in self.cast_wizard_spells.iter() {
            output.extend_from_slice(&key.to_le_bytes());
            output.extend(spell.as_output().unwrap());
        }
        output.extend_from_slice(&self.used_monster_abilities.len().to_le_bytes());
        for (key, spell) in self.used_monster_abilities.iter() {
            output.extend_from_slice(&key.to_le_bytes());
            output.extend(spell.as_output().unwrap());
        }
        output.extend_from_slice(&self.past_ticks.len().to_le_bytes());
        for tick in self.past_ticks.iter() {
            output.extend_from_slice(&tick.len().to_le_bytes());
            for event in tick.iter() {
                match event {
                    BattleEvent::Wizard(atom) | BattleEvent::Monster(atom) => {
                        output.push(match event {
                            BattleEvent::Wizard(_) => 0,
                            BattleEvent::Monster(_) => 1,
                            _ => 100,
                        });
                        Self::as_output_atom(atom, &mut output)?;
                    }
                    BattleEvent::Victory => { output.push(2); },
                    BattleEvent::Defeat => { output.push(3) },
                }
            }
        }
        Ok(output)
    }

    pub fn as_output_atom(atom: &BattleAtom, out: &mut Vec<u8>) -> std::io::Result<()>{
        use byteorder::{LittleEndian, WriteBytesExt};
        match atom {
            BattleAtom::CastSpell(caster, spell) => {
                out.write_u8(0)?;
                out.extend_from_slice(&caster.to_le_bytes());
                out.extend(spell.as_output()?);
            }
            BattleAtom::FizzleSpell(caster, spell) => {
                out.write_u8(1)?;
                out.extend_from_slice(&caster.to_le_bytes());
                out.extend(spell.as_output()?);
            }
            BattleAtom::SpellEnd(caster, spell) => {
                out.write_u8(2)?;
                out.extend_from_slice(&caster.to_le_bytes());
                out.extend(spell.as_output()?);
            }
            BattleAtom::TickEffect(caster, spell, effect_index, progress) => {
                out.write_u8(3)?;
                out.extend_from_slice(&caster.to_le_bytes());
                out.extend(spell.as_output()?);
                out.push(*effect_index);
                out.write_u32::<LittleEndian>(*progress)?;
            }
            BattleAtom::Mutation(battle_mut) => {
                match battle_mut {
                    BattleMut::Damage(damager, damagee, damage, glyph) => {
                        out.write_u8(4)?;
                        out.extend_from_slice(&damager.to_le_bytes());
                        out.extend_from_slice(&damagee.to_le_bytes());
                        out.write_u16::<LittleEndian>(*damage)?;
                        out.push(match glyph {
                            Glyph::Fire => 0,
                            Glyph::Water => 1,
                            Glyph::Earth => 2,
                            Glyph::Air => 3,
                            Glyph::Void => 4,
                        });
                    }
                    BattleMut::Heal(healer, healee, heal) => {
                        out.write_u8(5)?;
                        out.extend_from_slice(&healer.to_le_bytes());
                        out.extend_from_slice(&healee.to_le_bytes());
                        out.write_u16::<LittleEndian>(*heal)?;
                    }
                    BattleMut::IncurStatus(statuser, statusee, status, value, duration) => {
                        out.write_u8(6)?;
                        out.extend_from_slice(&statuser.to_le_bytes());
                        out.extend_from_slice(&statusee.to_le_bytes());
                        out.push(match status {
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
                        out.write_u16::<LittleEndian>(*value)?;
                        out.write_u16::<LittleEndian>(*duration)?;
                    }
                    BattleMut::LoseStatus(statuser, statusee, status) => {
                        out.write_u8(7)?;
                        out.extend_from_slice(&statuser.to_le_bytes());
                        out.extend_from_slice(&statusee.to_le_bytes());
                        out.push(match status {
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
                out.write_u8(8)?;
                out.extend_from_slice(&killer.to_le_bytes());
                out.extend_from_slice(&killee.to_le_bytes());
            }
        }
        Ok(())
    }

    pub fn from_buf(buf: &[u8]) -> std::io::Result<Self> {
        use byteorder::ReadBytesExt;
        use std::io::Read;
        let mut buf = std::io::Cursor::new(buf);
        let mut active_allies = Vec::new();
        let mut active_enemies = Vec::new();
        let mut cast_wizard_spells = HashMap::new();
        let mut used_monster_abilities = HashMap::new();
        let mut past_ticks = Vec::new();
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf)?;
        let ally_count = usize::from_le_bytes(usize_buf);
        for _ in 0..ally_count {
            buf.read_exact(&mut usize_buf)?;
            let ally = usize::from_le_bytes(usize_buf);
            active_allies.push(ally);
        }
        buf.read_exact(&mut usize_buf)?;
        let enemy_count = usize::from_le_bytes(usize_buf);
        for _ in 0..enemy_count {
            buf.read_exact(&mut usize_buf)?;
            let enemy = usize::from_le_bytes(usize_buf);
            active_enemies.push(enemy);
        }
        buf.read_exact(&mut usize_buf)?;
        let cast_spell_count = usize::from_le_bytes(usize_buf);
        for _ in 0..cast_spell_count {
            buf.read_exact(&mut usize_buf)?;
            let caster = usize::from_le_bytes(usize_buf);
            let spell = Spell::from_buf(&mut buf)?;
            cast_wizard_spells.insert(caster, spell);
        }
        buf.read_exact(&mut usize_buf)?;
        let used_ability_count = usize::from_le_bytes(usize_buf);
        for _ in 0..used_ability_count {
            buf.read_exact(&mut usize_buf)?;
            let caster = usize::from_le_bytes(usize_buf);
            let spell = Spell::from_buf(&mut buf)?;
            used_monster_abilities.insert(caster, spell);
        }
        buf.read_exact(&mut usize_buf)?;
        let tick_count = usize::from_le_bytes(usize_buf);
        for _ in 0..tick_count {
            let mut tick = Vec::new();
            buf.read_exact(&mut usize_buf)?;
            let event_count = usize::from_le_bytes(usize_buf);
            for _ in 0..event_count {
                let event_type = buf.read_u8()?;
                let event = match event_type {
                    0 => BattleEvent::Wizard(Self::from_buf_atom(&mut buf)?),
                    1 => BattleEvent::Monster(Self::from_buf_atom(&mut buf)?),
                    2 => BattleEvent::Victory,
                    3 => BattleEvent::Defeat,
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid event type")),
                };
                tick.push(event);
            }
            past_ticks.push(tick);
        }
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

    pub fn from_buf_atom(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<BattleAtom> {
        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::Read;
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        let atom_type = buf.read_u8()?;
        match atom_type {
            0 => {
                buf.read_exact(&mut usize_buf)?;
                let caster = usize::from_le_bytes(usize_buf);
                let spell = Spell::from_buf(buf)?;
                Ok(BattleAtom::CastSpell(caster, spell))
            }
            1 => {
                buf.read_exact(&mut usize_buf)?;
                let caster = usize::from_le_bytes(usize_buf);
                let spell = Spell::from_buf(buf)?;
                Ok(BattleAtom::FizzleSpell(caster, spell))
            }
            2 => {
                buf.read_exact(&mut usize_buf)?;
                let caster = usize::from_le_bytes(usize_buf);
                let spell = Spell::from_buf(buf)?;
                Ok(BattleAtom::SpellEnd(caster, spell))
            }
            3 => {
                buf.read_exact(&mut usize_buf)?;
                let caster = usize::from_le_bytes(usize_buf);
                let spell = Spell::from_buf(buf)?;
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

impl Battle {
    pub fn tick_effect<T>(tick: &mut Tick, caster: &T, caster_as_target: &Target, spell: &Spell, effect_index: &u8, progress: &u32, battle_ticks: &mut Tick, ally_targets: &mut Vec<Target>, enemy_targets: &mut Vec<Target>) 
        where T : BattleEntity + Idable,
            Colosseum: ColosseumArena<T> {
        if let Some(effect) = spell.effect(*effect_index) {
            match spell.target() {
                TargetType::Ally(num) => {
                    Self::sort_spell_by_targets(spell, enemy_targets);
                    for target in ally_targets.iter().take(*num as usize) {
                        battle_ticks.push(caster.as_event(Self::effect(&caster_as_target, target, &effect, &spell.glyph)));
                    }
                },
                TargetType::MeAlone => {
                    battle_ticks.push(
                        BattleEvent::Monster(Self::effect(
                            &caster_as_target, 
                            &caster_as_target, &effect, &spell.glyph)));
                },
                TargetType::Enemy(num) => {
                    Self::sort_spell_by_targets(spell, enemy_targets);
                    for target in enemy_targets.iter().take(*num as usize) {
                        battle_ticks.push(BattleEvent::Wizard(Self::effect(&caster_as_target, target, &effect, &spell.glyph)));
                    }
                },
            };
            
            
            if effect.done(*progress as u16 + 1) {
                if let Some(_) = spell.effect(*effect_index + 1) {
                    tick.push(caster.as_event(BattleAtom::TickEffect(caster_as_target.index, spell.clone(), *effect_index + 1, 0)));
                } else {
                    tick.push(caster.as_event(BattleAtom::SpellEnd(caster_as_target.index, spell.clone())));
                }
            } else {
                tick.push(caster.as_event(BattleAtom::TickEffect(caster_as_target.index, spell.clone(), *effect_index, *progress + 1)));
            }
        }
    }
    

    fn effect(caster: &Target, target: &Target, effect: &Effect, glyph: &(Glyph, u16),) -> BattleAtom {
        match &effect.application {
            EffectApplication::Damage => {
                    let damage = effect.value + (caster.augment.val16(&glyph.0) + caster.augment_cast.val16(&glyph.0)) * glyph.1;
                    let resist = target.resist.val16(&glyph.0) * glyph.1;
                    if resist > damage + 1 {
                        BattleAtom::Mutation(BattleMut::Damage(caster.index, target.index, 1, glyph.0.clone()))
                    } else {
                        BattleAtom::Mutation(BattleMut::Damage(caster.index, target.index, damage - resist, glyph.0.clone()))
                    }
            }
            EffectApplication::Heal => 
                BattleAtom::Mutation(BattleMut::Heal(caster.index, target.index, effect.value + (caster.augment.val16(&glyph.0) + caster.augment_cast.val16(&glyph.0) + target.augment.val16(&glyph.0)) * glyph.1)),
            EffectApplication::Status(status, duration) => {
                let value = effect.value 
                    + (caster.augment.val16(&glyph.0) + caster.augment_cast.val16(&glyph.0)) * glyph.1;
                let resist = target.resist.val16(&glyph.0) * glyph.1;
                if resist > value + 1 {
                    BattleAtom::Mutation(BattleMut::IncurStatus(caster.index, target.index, status.clone(), 1, *duration))
                } else {
                    BattleAtom::Mutation(BattleMut::IncurStatus(caster.index, target.index, status.clone(), value, *duration))
                }
            }
            EffectApplication::RemoveStatus(status) => BattleAtom::Mutation(BattleMut::LoseStatus(caster.index, target.index, status.clone()))
        }
    }

    pub fn mutate<T>(battle_mut: &BattleMut, target: &mut T, tick: &mut Tick) where T : BattleEntity {
        match battle_mut {
            BattleMut::Damage(damager, damagee, damage, _) => {
                target.set_hp(target.get_hp().saturating_sub(*damage as u32));
                if target.get_hp() == 0 {
                    tick.push(target.as_event(BattleAtom::Kill(*damager, *damagee)));
                }
            }
            BattleMut::Heal(_healer, _healee, heal) => {
                target.set_hp((target.get_hp() + *heal as u32).min(target.get_max_hp()));
            }
            BattleMut::IncurStatus(_statuser, _statusee, status, value, duration) => {
                target.get_status_mut().insert(status, *value, *duration);
            }
            BattleMut::LoseStatus(_statuser, _statusee, status) => {
                target.get_status_mut().remove(status);
            }
        }
    }
    
}

pub trait BattleEntity {
    fn get_hp(&self) -> u32;
    fn set_hp(&mut self, hp: u32);
    fn get_max_hp(&self) -> u32;
    fn get_status(&self) -> &StatusSet;
    fn get_status_mut(&mut self) -> &mut StatusSet;
    fn get_affinity(&self) -> &Affinity;
    fn augment(&self) -> &Affinity;
    fn augment_cast(&self) -> Affinity;
    fn resist(&self) -> &Affinity;
    fn as_event(&self, atom: BattleAtom) -> BattleEvent;
    fn as_enemy_event(&self, atom: BattleAtom) -> BattleEvent;

    fn as_target(&self, index: usize) -> Target {
        let health = match self.get_hp() * 100u32 / self.get_max_hp() {
            0 => TargetHealth::NoHealth,
            1..=25  => TargetHealth::LowHealth,
            26..=70 => TargetHealth::MediumHealth,
            71..=99 => TargetHealth::HighHealth,
            _ => TargetHealth::FullHealth,
        };
        let squishy = match self.get_max_hp() {
            0..=50 => TargetSqiushy::Squishy,
            51..=100 => TargetSqiushy::MidRange,
            _ => TargetSqiushy::Tanky,
        };
        Target {
            index,
            health,
            squishy,
            statuses: self.get_status().clone(),
            augment: self.augment().clone(),
            augment_cast: self.augment_cast().clone(),
            resist: self.resist().clone(),
        }
    }
}

impl BattleEntity for Wizard {
    fn get_hp(&self) -> u32 { self.hp }
    fn set_hp(&mut self, hp: u32) { self.hp = hp; }
    fn get_max_hp(&self) -> u32 { self.max_hp }
    fn get_status(&self) -> &StatusSet { &self.status }
    fn get_status_mut(&mut self) -> &mut StatusSet { &mut self.status }
    fn get_affinity(&self) -> &Affinity { &self.affinity }
    fn augment(&self) -> &Affinity { &self.affinity }
    fn augment_cast(&self) -> Affinity { self.spellbook_affinity().clone() }
    fn resist(&self) -> &Affinity { &self.affinity }
    fn as_event(&self, atom: BattleAtom) -> BattleEvent {
        BattleEvent::Wizard(atom)
    }
    fn as_enemy_event(&self, atom: BattleAtom) -> BattleEvent {
        BattleEvent::Monster(atom)
    }
}

impl BattleEntity for Monster {
    fn get_hp(&self) -> u32 { self.hp }
    fn set_hp(&mut self, hp: u32) { self.hp = hp; }
    fn get_max_hp(&self) -> u32 { self.max_hp }
    fn get_status(&self) -> &StatusSet { &self.status }
    fn get_status_mut(&mut self) -> &mut StatusSet { &mut self.status }
    fn get_affinity(&self) -> &Affinity { &self.affinity }
    fn augment(&self) -> &Affinity { &self.affinity }
    fn augment_cast(&self) -> Affinity { Affinity::new() }
    fn resist(&self) -> &Affinity { &self.affinity }
    fn as_event(&self, atom: BattleAtom) -> BattleEvent {
        BattleEvent::Monster(atom)
    }
    fn as_enemy_event(&self, atom: BattleAtom) -> BattleEvent {
        BattleEvent::Wizard(atom)
    }
}

impl Colosseum {
    fn as_target<T>(&self, id: Index, index: usize) -> Target where T : crate::a::e::col::Idable, T: BattleEntity, Colosseum: ColosseumArena<T> {
        let target = (self as &dyn ColosseumArena<T>).get(id);
        target.as_target(index)
    }
}