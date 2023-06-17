use crate::a::c::out::Outputable;

use super::{Glyph, Style, status::Status};


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PriorityType {
    Squishy,
    Tanky,
    LowHealth,
    HighHealth,
    HasStatus(Status),
    NoStatus(Status),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PriorityTypes {
    Single(PriorityType),
    Or(PriorityType, PriorityType),
    And(PriorityType, PriorityType),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum EffectDuration {
    OverTime(u16),
    Growth(u16, u16),
    AfterXTime(u16),
    Instant,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum EffectApplication {
    Damage,
    Heal,
    Status(Status, u16),
    RemoveStatus(Status),
}

impl PriorityType {
    pub fn from_u8(id: u8) -> std::io::Result<PriorityType> {
        match id {
            0 => Ok(PriorityType::Squishy),
            1 => Ok(PriorityType::Tanky),
            2 => Ok(PriorityType::LowHealth),
            3 => Ok(PriorityType::HighHealth),
            4..=17 => Ok(PriorityType::HasStatus(Status::from_u8(id - 4))),
            18..=32 => Ok(PriorityType::NoStatus(Status::from_u8(id - 18))),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid priority type byte")),
        }
    }
}

impl PriorityTypes {
    pub fn from_3u8(kind: u8, p1: u8, p2: u8) -> std::io::Result<Self> {
        match kind {
            0 => Ok(Self::Single(PriorityType::from_u8(p1)?)),
            1 => Ok(Self::Or(PriorityType::from_u8(p1)?, PriorityType::from_u8(p2)?)),
            2 => Ok(Self::And(PriorityType::from_u8(p1)?, PriorityType::from_u8(p2)?)),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid priority type byte")),
        }
    }
}

impl TargetType {
    pub fn from_2u8(kind: u8, index: u8) -> std::io::Result<Self> {
        match kind {
            0 => Ok(Self::MeAlone),
            1 => Ok(Self::Ally(index)),
            2 => Ok(Self::Enemy(index)),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid target type byte")),
        }
    }
}

impl EffectApplication {
    pub fn apply(&self, target: &mut Status, value: u16) {
        match self {
            EffectApplication::Damage => match target {
                Status::Barrier(_) => {
                    if value > 0 {
                        *target = Status::Barrier(Glyph::Void);
                    }
                }
                _ => {
                    if value > 0 {
                        *target = Status::Barrier(Glyph::Void);
                    }
                }
            },
            EffectApplication::Heal => match target {
                Status::Barrier(_) => {
                    if value > 0 {
                        *target = Status::Barrier(Glyph::Void);
                    }
                }
                _ => {
                    if value > 0 {
                        *target = Status::Barrier(Glyph::Void);
                    }
                }
            },
            EffectApplication::Status(_status, _duration) => match target {
                Status::Barrier(_) => {
                    if value > 0 {
                        *target = Status::Barrier(Glyph::Void);
                    }
                }
                _ => {
                    if value > 0 {
                        *target = Status::Barrier(Glyph::Void);
                    }
                }
            },
            EffectApplication::RemoveStatus(_status) => match target {
                Status::Barrier(_) => {
                    if value > 0 {
                        *target = Status::Barrier(Glyph::Void);
                    }
                }
                _ => {
                    if value > 0 {
                        *target = Status::Barrier(Glyph::Void);
                    }
                }
            },
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Effect {
    pub value: u16,
    pub duration: EffectDuration,
    pub application: EffectApplication,
}

impl Effect {
    pub const fn new(
        value: u16,
        duration: EffectDuration,
        application: EffectApplication,
    ) -> Effect {
        Effect {
            value: value,
            duration: duration,
            application: application,
        }
    }

    pub fn done(&self, progress_index: u16) -> bool {
        match self.duration {
            EffectDuration::OverTime(duration) => progress_index >= duration,
            EffectDuration::Growth(duration, _) => progress_index >= duration,
            EffectDuration::AfterXTime(duration) => progress_index >= duration,
            EffectDuration::Instant => progress_index >= 1,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum EffectProgression {
    None,
    Single(Effect),
    Duo(Effect, Effect),
    Trio(Effect, Effect, Effect),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Ability {
    pub(in super::super) priority: PriorityTypes,
    pub(in super::super) target: TargetType,
    pub(in super::super) effects: EffectProgression,
}

impl Ability {
    pub const fn exact(
        priority: PriorityTypes,
        target: TargetType,
        effects: EffectProgression,
    ) -> Ability {
        Ability {
            priority,
            target,
            effects,
        }
    }

    pub const fn single(priority: PriorityType, target: TargetType, effect: Effect) -> Ability {
        Ability {
            priority: PriorityTypes::Single(priority),
            target,
            effects: EffectProgression::Single(effect),
        }
    }

    pub const fn duo(
        priority: PriorityType,
        target: TargetType,
        effect1: Effect,
        effect2: Effect,
    ) -> Ability {
        Ability {
            priority: PriorityTypes::Single(priority),
            target,
            effects: EffectProgression::Duo(effect1, effect2),
        }
    }

    pub const fn len(&self) -> u8 {
        match &self.effects {
            EffectProgression::None => 0,
            EffectProgression::Single(_) => 1,
            EffectProgression::Duo(_, _) => 2,
            EffectProgression::Trio(_, _, _) => 3,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TargetType {
    MeAlone,
    Ally(u8),
    Enemy(u8),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Spell {
    pub name: &'static str,
    pub glyph: (Glyph, u16),
    pub style: (Style, u16),
    pub ability: Ability,
}

impl Spell {
    pub const fn new(
        name: &'static str,
        glyph: (Glyph, u16),
        style: (Style, u16),
        ability: Ability,
    ) -> Spell {
        Spell {
            name,
            glyph,
            style,
            ability,
        }
    }

    pub fn priorities(&self) -> Vec<Vec<&PriorityType>> {
        match &self.ability.priority {
            PriorityTypes::Single(priority) => vec![vec![priority]],
            PriorityTypes::And(priority1, priority2) => vec![vec![priority1, priority2]],
            PriorityTypes::Or(priority1, priority2) => vec![vec![priority1], vec![priority2]],
        }
    }

    pub fn priority_types(&self) -> PriorityTypes {
        self.ability.priority.clone()
    }

    pub fn target(&self) -> &TargetType {
        &self.ability.target
    }

    pub fn effect(&self, effect_index: u8) -> Option<&Effect> {
        match &self.ability.effects {
            EffectProgression::None => None,
            EffectProgression::Single(effect) => {
                if effect_index == 1 {
                    Some(effect)
                } else {
                    None
                }
            }
            EffectProgression::Duo(effect1, effect2) => match effect_index {
                1 => Some(effect1),
                2 => Some(effect2),
                _ => None,
            },
            EffectProgression::Trio(effect1, effect2, effect3) => match effect_index {
                1 => Some(effect1),
                2 => Some(effect2),
                3 => Some(effect3),
                _ => None,
            },
        }
    }
}

pub mod spells {
    use super::{Glyph, Spell, Style};
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        pub static ref BY_GLYPH: HashMap<Glyph, Vec<Spell>> = {
            let mut spells = HashMap::with_capacity(5);
            spells.insert(
                Glyph::Fire,
                vec![
                    fire::FIREBALL,
                    fire::RAGE,
                    fire::BURN,
                    fire::FLAME_WALL,
                    fire::FLAME_BLAST,
                    fire::FIRESTORM,
                ],
            );
            spells.insert(
                Glyph::Water,
                vec![water::HEAL, water::FLUID, water::SUBMERGE],
            );
            spells.insert(
                Glyph::Earth,
                vec![
                    earth::EARTH_BARRIER,
                    earth::STUN,
                    earth::HARDEN,
                    earth::EARTHQUAKE,
                ],
            );
            spells.insert(Glyph::Air, vec![air::FLY, air::LIGHTNING, air::SHOCK]);
            spells.insert(
                Glyph::Void,
                vec![void::UNENDING_HUNGER, void::MAGIC_MISSILE],
            );
            spells
        };
        pub static ref BY_GLYPH_AND_STYLE: HashMap<Glyph, HashMap<Style, Vec<Spell>>> = {
            let mut spells = HashMap::with_capacity(5);
            let mut fire = HashMap::with_capacity(5);
            fire.insert(Style::Elder, vec![fire::BURN, fire::FLAME_BLAST]);
            fire.insert(Style::Arcane, vec![fire::FIREBALL]);
            fire.insert(Style::Ancient, vec![fire::FLAME_WALL]);
            fire.insert(Style::Eldrich, vec![fire::RAGE, fire::FIRESTORM]);
            fire.insert(Style::Void, vec![]);

            let mut water = HashMap::with_capacity(5);
            water.insert(Style::Elder, vec![water::FLUID]);
            water.insert(Style::Arcane, vec![]);
            water.insert(Style::Ancient, vec![water::HEAL]);
            water.insert(Style::Eldrich, vec![water::SUBMERGE]);
            water.insert(Style::Void, vec![]);

            let mut earth = HashMap::with_capacity(5);
            earth.insert(Style::Elder, vec![earth::EARTH_BARRIER]);
            earth.insert(Style::Arcane, vec![earth::STUN]);
            earth.insert(Style::Ancient, vec![earth::HARDEN]);
            earth.insert(Style::Eldrich, vec![earth::EARTHQUAKE]);
            earth.insert(Style::Void, vec![]);

            let mut air = HashMap::with_capacity(5);
            air.insert(Style::Elder, vec![air::FLY]);
            air.insert(Style::Arcane, vec![air::LIGHTNING]);
            air.insert(Style::Ancient, vec![air::SHOCK]);
            air.insert(Style::Eldrich, vec![]);
            air.insert(Style::Void, vec![]);

            let mut void = HashMap::with_capacity(5);
            void.insert(Style::Elder, vec![]);
            void.insert(Style::Arcane, vec![void::MAGIC_MISSILE]);
            void.insert(Style::Ancient, vec![]);
            void.insert(Style::Eldrich, vec![void::UNENDING_HUNGER]);
            void.insert(Style::Void, vec![]);

            spells.insert(Glyph::Fire, fire);
            spells.insert(Glyph::Water, water);
            spells.insert(Glyph::Earth, earth);
            spells.insert(Glyph::Air, air);
            spells.insert(Glyph::Void, void);
            spells
        };
        pub static ref BY_STYLE: HashMap<Style, Vec<Spell>> = {
            let mut spells = HashMap::with_capacity(5);
            spells.insert(
                Style::Elder,
                vec![
                    fire::BURN,
                    fire::FLAME_BLAST,
                    water::HEAL,
                    earth::EARTH_BARRIER,
                    air::FLY,
                ],
            );
            spells.insert(Style::Arcane, vec![fire::FIREBALL, air::LIGHTNING]);
            spells.insert(
                Style::Ancient,
                vec![fire::FLAME_WALL, water::HEAL, earth::HARDEN],
            );
            spells.insert(
                Style::Eldrich,
                vec![
                    fire::RAGE,
                    fire::FIRESTORM,
                    water::SUBMERGE,
                    earth::EARTHQUAKE,
                ],
            );
            spells.insert(
                Style::Void,
                vec![void::UNENDING_HUNGER, void::MAGIC_MISSILE],
            );
            spells
        };

        pub static ref ID_BY_NAME: HashMap<&'static str, usize> = {
            use super::super::mon::abilities;
            let mut ids = HashMap::with_capacity(128);
            ids.insert(fire::FIREBALL.name, 0);
            ids.insert(fire::RAGE.name, 1);
            ids.insert(fire::BURN.name, 2);
            ids.insert(fire::FLAME_WALL.name, 3);
            ids.insert(fire::FLAME_BLAST.name, 4);
            ids.insert(fire::FIRESTORM.name, 5);
            ids.insert(water::HEAL.name, 6);
            ids.insert(water::FLUID.name, 7);
            ids.insert(water::SUBMERGE.name, 8);
            ids.insert(earth::EARTH_BARRIER.name, 9);
            ids.insert(earth::STUN.name, 10);
            ids.insert(earth::HARDEN.name, 11);
            ids.insert(earth::EARTHQUAKE.name, 12);
            ids.insert(air::FLY.name, 13);
            ids.insert(air::LIGHTNING.name, 14);
            ids.insert(air::SHOCK.name, 15);
            ids.insert(void::UNENDING_HUNGER.name, 16);
            ids.insert(void::MAGIC_MISSILE.name, 17);

            ids.insert(abilities::TROLL[0].name, 18);
            ids.insert(abilities::TROLL[1].name, 19);
            ids.insert(abilities::BEAR[0].name, 20);
            ids.insert(abilities::BEAR[1].name, 21);
            ids.insert(abilities::OGRE[0].name, 22);
            ids.insert(abilities::OGRE[1].name, 23);
            ids.insert(abilities::BAT[0].name, 24);
            ids.insert(abilities::SPIDER[0].name, 25);
            ids.insert(abilities::SNAKE[0].name, 26);
            ids.insert(abilities::SNAKE[1].name, 27);
            ids.insert(abilities::WOLF[0].name, 28);
            ids.insert(abilities::WOLF[1].name, 29);
            ids.insert(abilities::DIRE_WOLF[0].name, 30);
            ids.insert(abilities::DIRE_WOLF[1].name, 31);
            ids.insert(abilities::DIRE_WOLF[2].name, 32);
            ids.insert(abilities::HELLCAT[0].name, 33);
            ids.insert(abilities::HELLCAT[1].name, 34);
            ids.insert(abilities::DEMON[0].name, 35);
            ids.insert(abilities::DEMON[1].name, 36);
            ids.insert(abilities::DRAGON[0].name, 37);
            ids.insert(abilities::DRAGON[1].name, 38);
            ids.insert(abilities::GOBLIN[0].name, 39);
            ids.insert(abilities::ORC[0].name, 40);
            ids.insert(abilities::ORC[1].name, 41);
            ids.insert(abilities::UNDEAD_GOBLIN[0].name, 42);
            ids.insert(abilities::UNDEAD_ORC[0].name, 43);
            ids.insert(abilities::UNDEAD_HUMAN[0].name, 44);
            ids.insert(abilities::UNDEAD_TROLL[0].name, 45);
            ids.insert(abilities::ANCIENT_CONSTRUCT[0].name, 46);
            ids.insert(abilities::ANCIENT_CONSTRUCT[1].name, 47);
            ids.insert(abilities::ANGEL[0].name, 48);
            ids.insert(abilities::ANGEL[1].name, 49);
            ids.insert(abilities::ARCHON[0].name, 50);
            ids.insert(abilities::ARCHON[1].name, 51);
            ids.insert(abilities::TEMPLAR[0].name, 52);
            ids.insert(abilities::TEMPLAR[1].name, 53);
            ids.insert(abilities::ELEMENTAL[0].name, 54);
            ids.insert(abilities::ELEMENTAL[1].name, 55);
            ids.insert(abilities::ELEMENTAL[2].name, 56);
            ids.insert(abilities::ELEMENTAL[3].name, 57);
            ids.insert(abilities::GUARDIAN[0].name, 58);
            ids.insert(abilities::GUARDIAN[1].name, 59);
            ids.insert(abilities::RAT[0].name, 60);
            ids.insert(abilities::SLIME[0].name, 61);
            ids.insert(abilities::VOIDLING[0].name, 62);
            ids.insert(abilities::VOID_SPAWN[0].name, 63);
            ids.insert(abilities::VOID_SPAWN[1].name, 64);
            ids.insert(abilities::VOID_WALKER[0].name, 65);
            ids.insert(abilities::VOID_WALKER[1].name, 66);
            ids.insert(abilities::FALLEN_ANGEL[0].name, 67);
            ids.insert(abilities::FALLEN_ANGEL[1].name, 68);
            ids.insert(abilities::HUMAN[0].name, 69);
            ids.insert(abilities::CONSTRUCT[0].name, 70);
            ids.insert(abilities::CONSTRUCT[1].name, 71);

            ids
        };

        pub static ref NAME_BY_ID: Vec<&'static str> = {
            use super::super::mon::abilities;
            let mut names = Vec::with_capacity(128);
            names.push(fire::FIREBALL.name);
            names.push(fire::RAGE.name);
            names.push(fire::BURN.name);
            names.push(fire::FLAME_WALL.name);
            names.push(fire::FLAME_BLAST.name);
            names.push(fire::FIRESTORM.name);
            names.push(water::HEAL.name);
            names.push(water::FLUID.name);
            names.push(water::SUBMERGE.name);
            names.push(earth::EARTH_BARRIER.name);
            names.push(earth::STUN.name);
            names.push(earth::HARDEN.name);
            names.push(earth::EARTHQUAKE.name);
            names.push(air::FLY.name);
            names.push(air::LIGHTNING.name);
            names.push(air::SHOCK.name);
            names.push(void::UNENDING_HUNGER.name);
            names.push(void::MAGIC_MISSILE.name);

            names.push(abilities::TROLL[0].name);
            names.push(abilities::TROLL[1].name);
            names.push(abilities::BEAR[0].name);
            names.push(abilities::BEAR[1].name);
            names.push(abilities::OGRE[0].name);
            names.push(abilities::OGRE[1].name);
            names.push(abilities::BAT[0].name);
            names.push(abilities::SPIDER[0].name);
            names.push(abilities::SNAKE[0].name);
            names.push(abilities::SNAKE[1].name);
            names.push(abilities::WOLF[0].name);
            names.push(abilities::WOLF[1].name);
            names.push(abilities::DIRE_WOLF[0].name);
            names.push(abilities::DIRE_WOLF[1].name);
            names.push(abilities::DIRE_WOLF[2].name);
            names.push(abilities::HELLCAT[0].name);
            names.push(abilities::HELLCAT[1].name);
            names.push(abilities::DEMON[0].name);
            names.push(abilities::DEMON[1].name);
            names.push(abilities::DRAGON[0].name);
            names.push(abilities::DRAGON[1].name);
            names.push(abilities::GOBLIN[0].name);
            names.push(abilities::ORC[0].name);
            names.push(abilities::ORC[1].name);
            names.push(abilities::UNDEAD_GOBLIN[0].name);
            names.push(abilities::UNDEAD_ORC[0].name);
            names.push(abilities::UNDEAD_HUMAN[0].name);
            names.push(abilities::UNDEAD_TROLL[0].name);
            names.push(abilities::ANCIENT_CONSTRUCT[0].name);
            names.push(abilities::ANCIENT_CONSTRUCT[1].name);
            names.push(abilities::ANGEL[0].name);
            names.push(abilities::ANGEL[1].name);
            names.push(abilities::ARCHON[0].name);
            names.push(abilities::ARCHON[1].name);
            names.push(abilities::TEMPLAR[0].name);
            names.push(abilities::TEMPLAR[1].name);
            names.push(abilities::ELEMENTAL[0].name);
            names.push(abilities::ELEMENTAL[1].name);
            names.push(abilities::ELEMENTAL[2].name);
            names.push(abilities::ELEMENTAL[3].name);
            names.push(abilities::GUARDIAN[0].name);
            names.push(abilities::GUARDIAN[1].name);
            names.push(abilities::RAT[0].name);
            names.push(abilities::SLIME[0].name);
            names.push(abilities::VOIDLING[0].name);
            names.push(abilities::VOID_SPAWN[0].name);
            names.push(abilities::VOID_SPAWN[1].name);
            names.push(abilities::VOID_WALKER[0].name);
            names.push(abilities::VOID_WALKER[1].name);
            names.push(abilities::FALLEN_ANGEL[0].name);
            names.push(abilities::FALLEN_ANGEL[1].name);
            names.push(abilities::HUMAN[0].name);
            names.push(abilities::CONSTRUCT[0].name);
            names.push(abilities::CONSTRUCT[1].name);

            names
        };
    }

    pub mod fire {
        use crate::a::c::e::spell::{EffectDuration, EffectApplication, Effect};

        use super::super::{
            Ability, EffectProgression, Glyph, PriorityType, PriorityTypes, Spell, Status, Style,
            TargetType,
        };
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        lazy_static! {
            pub static ref BY_NAME: HashMap<&'static str, Spell> = {
                let mut spells = HashMap::with_capacity(6);
                spells.insert("fireball", FIREBALL);
                spells.insert("Rage", RAGE);
                spells.insert("Burn", BURN);
                spells.insert("Flame wall", FLAME_WALL);
                spells.insert("Flame blast", FLAME_BLAST);
                spells.insert("firestorm", FIRESTORM);
                spells
            };
            pub static ref BY_STYLE: HashMap<Style, Vec<Spell>> = {
                let mut spells = HashMap::with_capacity(5);
                spells.insert(Style::Elder, vec![BURN, FLAME_BLAST]);
                spells.insert(Style::Arcane, vec![FIREBALL]);
                spells.insert(Style::Ancient, vec![FLAME_WALL]);
                spells.insert(Style::Eldrich, vec![RAGE, FIRESTORM]);
                spells
            };
        }

        pub const FIREBALL: Spell = Spell::new(
            "fireball",
            (Glyph::Fire, 1),
            (Style::Arcane, 1),
            Ability::duo(
                PriorityType::Squishy,
                TargetType::Enemy(1),
                Effect {
                    value: 5,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Damage,
                },
                Effect {
                    value: 3,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Burning, 4),
                },
            ),
        );

        pub const RAGE: Spell = Spell::new(
            "Rage",
            (Glyph::Fire, 1),
            (Style::Eldrich, 1),
            Ability::duo(
                PriorityType::NoStatus(Status::Raging),
                TargetType::MeAlone,
                Effect {
                    value: 1,
                    duration: EffectDuration::Growth(5, 3),
                    application: EffectApplication::Status(Status::Raging, 1),
                },
                Effect {
                    value: 1,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Weakened, 1),
                },
            ),
        );

        pub const BURN: Spell = Spell::new(
            "Burn",
            (Glyph::Fire, 1),
            (Style::Elder, 1),
            Ability::exact(
                PriorityTypes::Or(
                    PriorityType::NoStatus(Status::Burning),
                    PriorityType::HighHealth,
                ),
                TargetType::Enemy(4),
                EffectProgression::Single(Effect {
                    value: 4,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Burning, 5),
                }),
            ),
        );

        pub const FLAME_WALL: Spell = Spell::new(
            "Flame wall",
            (Glyph::Fire, 1),
            (Style::Ancient, 1),
            Ability::single(
                PriorityType::NoStatus(Status::Barrier(Glyph::Fire)),
                TargetType::Ally(1),
                Effect {
                    value: 3,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Barrier(Glyph::Fire), 8),
                },
            ),
        );

        pub const FLAME_BLAST: Spell = Spell::new(
            "Flame blast",
            (Glyph::Fire, 1),
            (Style::Elder, 1),
            Ability::duo(
                PriorityType::NoStatus(Status::Burning),
                TargetType::Enemy(4),
                Effect {
                    value: 4,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Damage,
                },
                Effect {
                    value: 1,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Burning, 3),
                },
            ),
        );

        pub const FIRESTORM: Spell = Spell::new(
            "firestorm",
            (Glyph::Fire, 3),
            (Style::Eldrich, 1),
            Ability::duo(
                PriorityType::NoStatus(Status::Burning),
                TargetType::Enemy(4),
                Effect {
                    value: 4,
                    duration: EffectDuration::Growth(3, 2),
                    application: EffectApplication::Damage,
                },
                Effect {
                    value: 1,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Burning, 3),
                },
            ),
        );
    }

    pub mod water {

        use super::super::{
            Ability, Effect, EffectApplication, EffectDuration, Glyph, PriorityType, Spell, Status,
            Style, TargetType,
        };
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        lazy_static! {
            pub static ref BY_NAME: HashMap<&'static str, Spell> = {
                let mut spells = HashMap::with_capacity(3);
                spells.insert("Heal", HEAL);
                spells.insert("Fluid", FLUID);
                spells.insert("Submerge", SUBMERGE);
                spells
            };
            pub static ref BY_STYLE: HashMap<Style, Vec<Spell>> = {
                let mut spells = HashMap::with_capacity(3);
                spells.insert(Style::Ancient, vec![HEAL]);
                spells.insert(Style::Elder, vec![FLUID]);
                spells.insert(Style::Eldrich, vec![SUBMERGE]);
                spells
            };
        }

        pub const HEAL: Spell = Spell::new(
            "Heal",
            (Glyph::Water, 1),
            (Style::Ancient, 1),
            Ability::duo(
                PriorityType::LowHealth,
                TargetType::Ally(1),
                Effect {
                    value: 3,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Heal,
                },
                Effect {
                    value: 3,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Fluid, 1),
                },
            ),
        );

        pub const FLUID: Spell = Spell::new(
            "Fluid",
            (Glyph::Water, 1),
            (Style::Elder, 1),
            Ability::single(
                PriorityType::NoStatus(Status::Fluid),
                TargetType::Ally(1),
                Effect {
                    value: 3,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Fluid, 5),
                },
            ),
        );

        pub const SUBMERGE: Spell = Spell::new(
            "Submerge",
            (Glyph::Water, 1),
            (Style::Eldrich, 1),
            Ability::single(
                PriorityType::NoStatus(Status::Submerged),
                TargetType::Enemy(1),
                Effect {
                    value: 1,
                    duration: EffectDuration::Growth(4, 3),
                    application: EffectApplication::Status(Status::Submerged, 3),
                },
            ),
        );
    }

    pub mod earth {

        use super::super::{
            Ability, Effect, EffectApplication, EffectDuration, Glyph, PriorityType, Spell, Status,
            Style, TargetType,
        };
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        lazy_static! {
            pub static ref BY_NAME: HashMap<&'static str, Spell> = {
                let mut spells = HashMap::with_capacity(5);
                spells.insert("earth barrier", EARTH_BARRIER);
                spells.insert("Stun", STUN);
                spells.insert("Harden", HARDEN);
                spells.insert("earthquake", EARTHQUAKE);
                spells
            };
            pub static ref BY_STYLE: HashMap<Style, Vec<Spell>> = {
                let mut spells = HashMap::with_capacity(5);
                spells.insert(Style::Elder, vec![EARTH_BARRIER]);
                spells.insert(Style::Void, vec![STUN]);
                spells.insert(Style::Ancient, vec![HARDEN]);
                spells
            };
        }

        pub const EARTHQUAKE: Spell = Spell::new(
            "earthquake",
            (Glyph::Earth, 3),
            (Style::Eldrich, 1),
            Ability::duo(
                PriorityType::NoStatus(Status::Stunned),
                TargetType::Enemy(4),
                Effect {
                    value: 1,
                    duration: EffectDuration::Growth(2, 2),
                    application: EffectApplication::Status(Status::Stunned, 1),
                },
                Effect {
                    value: 5,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Damage,
                },
            ),
        );

        pub const EARTH_BARRIER: Spell = Spell::new(
            "earth barrier",
            (Glyph::Earth, 1),
            (Style::Elder, 1),
            Ability::duo(
                PriorityType::NoStatus(Status::Barrier(Glyph::Earth)),
                TargetType::Ally(1),
                Effect {
                    value: 5,
                    duration: EffectDuration::Growth(3, 1),
                    application: EffectApplication::Status(Status::Barrier(Glyph::Earth), 1),
                },
                Effect {
                    value: 2,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Hardened, 1),
                },
            ),
        );

        pub const STUN: Spell = Spell::new(
            "Stun",
            (Glyph::Earth, 1),
            (Style::Void, 1),
            Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(1),
                Effect {
                    value: 1,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Stunned, 3),
                },
            ),
        );

        pub const HARDEN: Spell = Spell::new(
            "Harden",
            (Glyph::Earth, 1),
            (Style::Elder, 1),
            Ability::duo(
                PriorityType::NoStatus(Status::Hardened),
                TargetType::Ally(1),
                Effect {
                    value: 4,
                    duration: EffectDuration::Growth(4, 1),
                    application: EffectApplication::Status(Status::Hardened, 1),
                },
                Effect {
                    value: 1,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Barrier(Glyph::Earth), 1),
                },
            ),
        );
    }

    pub mod air {
        use crate::a::c::e::spell::EffectApplication;

        use super::super::{
            Ability, Effect, EffectDuration, Glyph, PriorityType, Spell, Status, Style, TargetType,
        };
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        lazy_static! {
            pub static ref BY_NAME: HashMap<&'static str, Spell> = {
                let mut spells = HashMap::with_capacity(5);
                spells.insert("Lightning", LIGHTNING);
                spells.insert("Shock", SHOCK);
                spells.insert("Fly", FLY);
                spells
            };
            pub static ref BY_STYLE: HashMap<Style, Vec<Spell>> = {
                let mut spells = HashMap::with_capacity(5);
                spells.insert(Style::Arcane, vec![LIGHTNING]);
                spells.insert(Style::Void, vec![SHOCK]);
                spells.insert(Style::Elder, vec![FLY]);
                spells
            };
        }

        pub const FLY: Spell = Spell::new(
            "Fly",
            (Glyph::Air, 1),
            (Style::Elder, 1),
            Ability::single(
                PriorityType::NoStatus(Status::Flying),
                TargetType::MeAlone,
                Effect {
                    value: 1,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Flying, 3),
                },
            ),
        );

        pub const LIGHTNING: Spell = Spell::new(
            "Lightning",
            (Glyph::Air, 1),
            (Style::Arcane, 1),
            Ability::duo(
                PriorityType::Squishy,
                TargetType::Enemy(1),
                Effect {
                    value: 4,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Damage,
                },
                Effect {
                    value: 2,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Shocked, 2),
                },
            ),
        );

        pub const SHOCK: Spell = Spell::new(
            "Shock",
            (Glyph::Air, 1),
            (Style::Void, 1),
            Ability::single(
                PriorityType::NoStatus(Status::Shocked),
                TargetType::Enemy(1),
                Effect {
                    value: 4,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Shocked, 4),
                },
            ),
        );
    }

    pub mod void {
        use crate::a::c::e::spell::{Effect, EffectApplication, EffectDuration};

        use super::super::{Ability, Glyph, PriorityType, Spell, Status, Style, TargetType};
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        lazy_static! {
            pub static ref BY_NAME: HashMap<&'static str, Spell> = {
                let mut spells = HashMap::with_capacity(5);
                spells.insert("Unending hunger", UNENDING_HUNGER);
                spells.insert("Magic Missile", MAGIC_MISSILE);
                spells
            };
            pub static ref BY_STYLE: HashMap<Style, Vec<Spell>> = {
                let mut spells = HashMap::with_capacity(5);
                spells.insert(Style::Eldrich, vec![UNENDING_HUNGER]);
                spells.insert(Style::Arcane, vec![MAGIC_MISSILE]);
                spells
            };
        }

        pub const UNENDING_HUNGER: Spell = Spell::new(
            "Unending hunger",
            (Glyph::Void, 1),
            (Style::Eldrich, 1),
            Ability::duo(
                PriorityType::HighHealth,
                TargetType::Enemy(1),
                Effect {
                    value: 1,
                    duration: EffectDuration::OverTime(10),
                    application: EffectApplication::Damage,
                },
                Effect {
                    value: 1,
                    duration: EffectDuration::Instant,
                    application: EffectApplication::Status(Status::Weakened, 2),
                },
            ),
        );

        pub const MAGIC_MISSILE: Spell = Spell::new(
            "Magic Missile",
            (Glyph::Void, 1),
            (Style::Arcane, 1),
            Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect {
                    value: 5,
                    duration: EffectDuration::AfterXTime(2),
                    application: EffectApplication::Damage,
                },
            ),
        );
    }
}
