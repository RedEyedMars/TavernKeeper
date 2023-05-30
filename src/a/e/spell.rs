use super::{Glyph, Style};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Status {
    Barrier(Glyph),
    Burning,
    Stunned,
    Submerged,
    Shocked,
    Weakened,
    Raging,
    Hardened,
    Fluid,
    Flying,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PriorityType {
    Squishy,
    Tanky,
    LowHealth,
    HighHealth,
    HasStatus(Status),
    NoStatus(Status),
    Or(Box<PriorityType>, Box<PriorityType>),
    And(Box<PriorityType>, Box<PriorityType>),
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
    value: u16,
    duration: EffectDuration,
    application: EffectApplication,
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
    priority: PriorityTypes,
    target: TargetType,
    effects: EffectProgression,
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
    pub glyph: (Glyph, u8),
    pub style: (Style, u8),
    pub ability: Ability,
}

impl Spell {
    pub const fn new(
        name: &'static str,
        glyph: (Glyph, u8),
        style: (Style, u8),
        ability: Ability,
    ) -> Spell {
        Spell {
            name,
            glyph,
            style,
            ability,
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
            spells.insert(Glyph::Fire, fire::BY_STYLE.clone());
            spells.insert(Glyph::Water, water::BY_STYLE.clone());
            spells.insert(Glyph::Earth, earth::BY_STYLE.clone());
            spells.insert(Glyph::Air, air::BY_STYLE.clone());
            spells.insert(Glyph::Void, void::BY_STYLE.clone());
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
    }

    pub mod fire {
        use crate::a::e::spell::{Effect, EffectApplication, EffectDuration};

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
        use crate::a::e::spell::EffectApplication;

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
        use crate::a::e::spell::{Effect, EffectApplication, EffectDuration};

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
