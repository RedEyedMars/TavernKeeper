use super::spell::Spell;
use super::wiz::{Acceptance, Affinity};
use super::status::StatusSet;
use super::{Style, Glyph};
use crate::generational_arena::Index;
use lazy_static::lazy_static;
use map_macro::hash_map;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum MonsterType {
    Troll, // Elder
    Dragon,
    Demon,
    Elemental,
    Ogre,
    Goblin,
    Orc,
    Human,
    Hellcat,
    Rat,
    Slime,
    Spider,
    Snake,
    DireWolf,
    Wolf,
    Bear,
    Bat,
    AncientConstruct,
    Construct,
    UndeadHuman,
    UndeadOrc,
    UndeadGoblin,
    UndeadTroll,
    Guardian,
    Voidling,
    VoidSpawn,
    VoidWalker,
    VoidLord,
    Templar,
    Archon,
    FallenAngel,
    Angel,
}

pub const ALL: [MonsterType; 32] = [
    MonsterType::Troll,
    MonsterType::Dragon,
    MonsterType::Demon,
    MonsterType::Elemental,
    MonsterType::Ogre,
    MonsterType::Goblin,
    MonsterType::Orc,
    MonsterType::Human,
    MonsterType::Hellcat,
    MonsterType::Rat,
    MonsterType::Slime,
    MonsterType::Spider,
    MonsterType::Snake,
    MonsterType::DireWolf,
    MonsterType::Wolf,
    MonsterType::Bear,
    MonsterType::Bat,
    MonsterType::AncientConstruct,
    MonsterType::Construct,
    MonsterType::UndeadHuman,
    MonsterType::UndeadOrc,
    MonsterType::UndeadGoblin,
    MonsterType::UndeadTroll,
    MonsterType::Guardian,
    MonsterType::Voidling,
    MonsterType::VoidSpawn,
    MonsterType::VoidWalker,
    MonsterType::VoidLord,
    MonsterType::Templar,
    MonsterType::Archon,
    MonsterType::FallenAngel,
    MonsterType::Angel,
];

lazy_static! {
    pub static ref BY_NAME: HashMap<&'static str, MonsterType> = hash_map! {
        "Dragon" => MonsterType::Dragon,
        "Demon" => MonsterType::Demon,
        "Elemental" => MonsterType::Elemental,
        "Ogre" => MonsterType::Ogre,
        "Goblin" => MonsterType::Goblin,
        "Orc" => MonsterType::Orc,
        "Human" => MonsterType::Human,
        "Hellcat" => MonsterType::Hellcat,
        "Rat" => MonsterType::Rat,
        "Slime" => MonsterType::Slime,
        "Spider" => MonsterType::Spider,
        "Snake" => MonsterType::Snake,
        "DireWolf" => MonsterType::DireWolf,
        "Wolf" => MonsterType::Wolf,
        "Bear" => MonsterType::Bear,
        "Bat" => MonsterType::Bat,
        "AncientConstruct" => MonsterType::AncientConstruct,
        "Construct" => MonsterType::Construct,
        "UndeadHuman" => MonsterType::UndeadHuman,
        "UndeadOrc" => MonsterType::UndeadOrc,
        "UndeadGoblin" => MonsterType::UndeadGoblin,
        "UndeadTroll" => MonsterType::UndeadTroll,
        "Guardian" => MonsterType::Guardian,
        "Voidling" => MonsterType::Voidling,
        "VoidSpawn" => MonsterType::VoidSpawn,
        "VoidWalker" => MonsterType::VoidWalker,
        "VoidLord" => MonsterType::VoidLord,
        "Templar" => MonsterType::Templar,
        "Archon" => MonsterType::Archon,
        "FallenAngel" => MonsterType::FallenAngel,
        "Angel" => MonsterType::Angel,
    };
}

impl MonsterType {
    pub fn ability_with_style(monster_type: MonsterType, style: &Style) -> Vec<Spell> {
        let mut abilities = abilities::BY_TYPE.get(&monster_type).unwrap().clone();
        abilities.iter_mut().for_each(|spell| {
            spell.style = (style.clone(), spell.style.1);
        });
        abilities
    }

    pub fn difficulty(&self) -> u8 {
        match self {
            Self::Troll => 4,
            Self::Bear => 5,
            Self::Bat => 1,
            Self::Spider => 1,
            Self::Snake => 2,
            Self::Wolf => 2,
            Self::DireWolf => 3,
            Self::Hellcat => 3,
            Self::Demon => 5,
            Self::Dragon => 6,
            Self::Ogre => 4,
            Self::Goblin => 1,
            Self::Orc => 2,
            Self::UndeadGoblin => 1,
            Self::UndeadHuman => 2,
            Self::UndeadOrc => 2,
            Self::UndeadTroll => 3,
            Self::AncientConstruct => 4,
            Self::Angel => 5,
            Self::Archon => 6,
            Self::Templar => 5,
            Self::Elemental => 4,
            Self::Guardian => 5,
            Self::Rat => 1,
            Self::Slime => 1,
            Self::Voidling => 2,
            Self::VoidWalker => 4,
            Self::VoidSpawn => 3,
            Self::VoidLord => 6,
            Self::FallenAngel => 5,
            Self::Human => 2,
            Self::Construct => 3,
        }
    }

    pub fn hp(&self, difficulty: u32) -> u32 {
        difficulty * match self {
            Self::Troll => 20,
            Self::Bear => 25,
            Self::Bat => 10,
            Self::Spider => 10,
            Self::Snake => 15,
            Self::Wolf => 15,
            Self::DireWolf => 20,
            Self::Hellcat => 20,
            Self::Demon => 25,
            Self::Dragon => 30,
            Self::Ogre => 20,
            Self::Goblin => 10,
            Self::Orc => 15,
            Self::UndeadGoblin => 10,
            Self::UndeadHuman => 15,
            Self::UndeadOrc => 15,
            Self::UndeadTroll => 20,
            Self::AncientConstruct => 20,
            Self::Angel => 25,
            Self::Archon => 30,
            Self::Templar => 25,
            Self::Elemental => 20,
            Self::Guardian => 25,
            Self::Rat => 10,
            Self::Slime => 10,
            Self::Voidling => 15,
            Self::VoidWalker => 20,
            Self::VoidSpawn => 20,
            Self::VoidLord => 30,
            Self::FallenAngel => 25,
            Self::Human => 15,
            Self::Construct => 20,
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Troll => 0,
            Self::Bear => 1,
            Self::Bat => 2,
            Self::Spider => 3,
            Self::Snake => 4,
            Self::Wolf => 5,
            Self::DireWolf => 6,
            Self::Hellcat => 7,
            Self::Demon => 8,
            Self::Dragon => 9,
            Self::Ogre => 10,
            Self::Goblin => 11,
            Self::Orc => 12,
            Self::UndeadGoblin => 13,
            Self::UndeadHuman => 14,
            Self::UndeadOrc => 15,
            Self::UndeadTroll => 16,
            Self::AncientConstruct => 17,
            Self::Angel => 18,
            Self::Archon => 19,
            Self::Templar => 20,
            Self::Elemental => 21,
            Self::Guardian => 22,
            Self::Rat => 23,
            Self::Slime => 24,
            Self::Voidling => 25,
            Self::VoidWalker => 26,
            Self::VoidSpawn => 27,
            Self::VoidLord => 28,
            Self::FallenAngel => 29,
            Self::Human => 30,
            Self::Construct => 31,
        }
    }

    pub fn from_u8(kind: u8) -> MonsterType {
        match kind {
            0 => Self::Troll,
            1 => Self::Bear,
            2 => Self::Bat,
            3 => Self::Spider,
            4 => Self::Snake,
            5 => Self::Wolf,
            6 => Self::DireWolf,
            7 => Self::Hellcat,
            8 => Self::Demon,
            9 => Self::Dragon,
            10 => Self::Ogre,
            11 => Self::Goblin,
            12 => Self::Orc,
            13 => Self::UndeadGoblin,
            14 => Self::UndeadHuman,
            15 => Self::UndeadOrc,
            16 => Self::UndeadTroll,
            17 => Self::AncientConstruct,
            18 => Self::Angel,
            19 => Self::Archon,
            20 => Self::Templar,
            21 => Self::Elemental,
            22 => Self::Guardian,
            23 => Self::Rat,
            24 => Self::Slime,
            25 => Self::Voidling,
            26 => Self::VoidWalker,
            27 => Self::VoidSpawn,
            28 => Self::VoidLord,
            29 => Self::FallenAngel,
            30 => Self::Human,
            31 => Self::Construct,
            _ => panic!("Invalid monster type"),
        }
    }
}
pub mod abilities {
    use super::super::{spell::{
        Ability, Effect, EffectApplication, EffectDuration, PriorityType, Spell, spells,
        TargetType,EffectProgression, PriorityTypes
    }, status::Status};
    use super::super::{Glyph, Style};
    use super::MonsterType;
    use lazy_static::lazy_static;
    use map_macro::hash_map;
    use std::collections::HashMap;

    lazy_static! {
        pub static ref BY_TYPE: HashMap<MonsterType, Vec<Spell>> = hash_map! {
            MonsterType::Troll => TROLL.to_vec(),
            MonsterType::Bear => BEAR.to_vec(),
            MonsterType::Bat => BAT.to_vec(),
            MonsterType::Spider => SPIDER.to_vec(),
            MonsterType::Snake => SNAKE.to_vec(),
            MonsterType::Wolf => WOLF.to_vec(),
            MonsterType::DireWolf => DIRE_WOLF.to_vec(),
            MonsterType::Hellcat => HELLCAT.to_vec(),
            MonsterType::Demon => DEMON.to_vec(),
            MonsterType::Dragon => DRAGON.to_vec(),
            MonsterType::Ogre => OGRE.to_vec(),
            MonsterType::Goblin => GOBLIN.to_vec(),
            MonsterType::Orc => ORC.to_vec(),
            MonsterType::UndeadGoblin => UNDEAD_GOBLIN.to_vec(),
            MonsterType::UndeadHuman => UNDEAD_HUMAN.to_vec(),
            MonsterType::UndeadOrc => UNDEAD_ORC.to_vec(),
            MonsterType::UndeadTroll => UNDEAD_TROLL.to_vec(),
            MonsterType::AncientConstruct => ANCIENT_CONSTRUCT.to_vec(),
            MonsterType::Angel => ANGEL.to_vec(),
            MonsterType::Archon => ARCHON.to_vec(),
            MonsterType::Templar => TEMPLAR.to_vec(),
            MonsterType::Elemental => ELEMENTAL.to_vec(),
            MonsterType::Guardian => GUARDIAN.to_vec(),
            MonsterType::Rat => RAT.to_vec(),
            MonsterType::Slime => SLIME.to_vec(),
            MonsterType::Voidling => VOIDLING.to_vec(),
            MonsterType::VoidWalker => VOID_WALKER.to_vec(),
            MonsterType::VoidSpawn => VOID_SPAWN.to_vec(),
            MonsterType::VoidLord => VOID_LORD.to_vec(),
            MonsterType::FallenAngel => FALLEN_ANGEL.to_vec(),
            MonsterType::Human => HUMAN.to_vec(),
            MonsterType::Construct => CONSTRUCT.to_vec(),
        };
    }

    pub const TROLL: [Spell; 2] = [
        Spell {
            name: "Bash",
            glyph: (Glyph::Earth, 1),
            style: (Style::Void, 1),
            ability: Ability::duo(
                PriorityType::LowHealth,
                TargetType::Enemy(2),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
                Effect::new(
                    1,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Stunned, 1),
                ),
            ),
        },
        Spell {
            name: "Bite",
            glyph: (Glyph::Earth, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
    ];

    pub const BEAR: [Spell; 2] = [
        Spell {
            name: "Bite",
            glyph: (Glyph::Earth, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Swipe",
            glyph: (Glyph::Earth, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(3),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
    ];

    pub const BAT: [Spell; 1] = [Spell {
        name: "Bite",
        glyph: (Glyph::Air, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::LowHealth,
            TargetType::Enemy(1),
            Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const SPIDER: [Spell; 1] = [Spell {
        name: "Bite",
        glyph: (Glyph::Earth, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::LowHealth,
            TargetType::Enemy(1),
            Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const SNAKE: [Spell; 2] = [
        Spell {
            name: "Bite",
            glyph: (Glyph::Fire, 1),
            style: (Style::Void, 1),
            ability: Ability::duo(
                PriorityType::Tanky,
                TargetType::Enemy(1),
                Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
                Effect::new(
                    1,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Burning, 3),
                ),
            ),
        },
        Spell {
            name: "Constrict",
            glyph: (Glyph::Air, 1),
            style: (Style::Void, 1),
            ability: Ability::duo(
                PriorityType::Squishy,
                TargetType::Enemy(1),
                Effect::new(3, EffectDuration::Growth(3, 1), EffectApplication::Damage),
                Effect::new(
                    1,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Weakened, 1),
                ),
            ),
        },
    ];

    pub const WOLF: [Spell; 2] = [
        Spell {
            name: "Bite",
            glyph: (Glyph::Earth, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Howl",
            glyph: (Glyph::Air, 1),
            style: (Style::Void, 1),
            ability: Ability::exact(
                PriorityTypes::Or(
                    PriorityType::NoStatus(Status::Raging),
                    PriorityType::HasStatus(Status::Weakened),
                ),
                TargetType::Ally(3),
                EffectProgression::Single(Effect::new(
                    3,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Raging, 1),
                )),
            ),
        },
    ];

    pub const DIRE_WOLF: [Spell; 3] = [
        Spell {
            name: "Bite",
            glyph: (Glyph::Earth, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect::new(10, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Howl",
            glyph: (Glyph::Air, 1),
            style: (Style::Void, 1),
            ability: Ability::exact(
                PriorityTypes::Or(
                    PriorityType::NoStatus(Status::Raging),
                    PriorityType::HasStatus(Status::Weakened),
                ),
                TargetType::Ally(3),
                EffectProgression::Single(Effect::new(
                    5,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Raging, 1),
                )),
            ),
        },
        Spell {
            name: "Swipe",
            glyph: (Glyph::Earth, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(3),
                Effect::new(4, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
    ];

    pub const HELLCAT: [Spell; 2] = [
        Spell {
            name: "Bite",
            glyph: (Glyph::Fire, 1),
            style: (Style::Void, 1),
            ability: Ability::duo(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect::new(7, EffectDuration::Instant, EffectApplication::Damage),
                Effect::new(
                    1,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Burning, 5),
                ),
            ),
        },
        Spell {
            name: "Tail whip",
            glyph: (Glyph::Fire, 1),
            style: (Style::Void, 1),
            ability: Ability::duo(
                PriorityType::LowHealth,
                TargetType::Enemy(3),
                Effect::new(4, EffectDuration::Instant, EffectApplication::Damage),
                Effect::new(
                    2,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Burning, 3),
                ),
            ),
        },
    ];

    pub const DEMON: [Spell; 2] = [spells::fire::FIREBALL, spells::fire::FIRESTORM];

    pub const DRAGON: [Spell; 2] = [
        Spell {
            name: "Bite",
            glyph: (Glyph::Fire, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect::new(10, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Fire Breath",
            glyph: (Glyph::Fire, 3),
            style: (Style::Void, 1),
            ability: Ability::duo(
                PriorityType::NoStatus(Status::Burning),
                TargetType::Enemy(8),
                Effect::new(10, EffectDuration::Instant, EffectApplication::Damage),
                Effect::new(
                    3,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Burning, 5),
                ),
            ),
        },
    ];

    pub const OGRE: [Spell; 2] = [
        Spell {
            name: "Clobber",
            glyph: (Glyph::Earth, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Smash",
            glyph: (Glyph::Earth, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(3),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
    ];

    pub const GOBLIN: [Spell; 1] = [Spell {
        name: "Stab",
        glyph: (Glyph::Air, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::Squishy,
            TargetType::Enemy(1),
            Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const ORC: [Spell; 2] = [
        Spell {
            name: "Stab",
            glyph: (Glyph::Air, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(1),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Slash",
            glyph: (Glyph::Air, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::HighHealth,
                TargetType::Enemy(1),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
    ];

    pub const UNDEAD_GOBLIN: [Spell; 1] = [Spell {
        name: "Stab",
        glyph: (Glyph::Air, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::Squishy,
            TargetType::Enemy(1),
            Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const UNDEAD_HUMAN: [Spell; 1] = [Spell {
        name: "Stab",
        glyph: (Glyph::Air, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::Squishy,
            TargetType::Enemy(1),
            Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const UNDEAD_ORC: [Spell; 1] = [Spell {
        name: "Stab",
        glyph: (Glyph::Air, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::Squishy,
            TargetType::Enemy(1),
            Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const UNDEAD_TROLL: [Spell; 1] = [Spell {
        name: "Bash",
        glyph: (Glyph::Earth, 1),
        style: (Style::Void, 1),
        ability: Ability::duo(
            PriorityType::LowHealth,
            TargetType::Enemy(2),
            Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            Effect::new(
                1,
                EffectDuration::Instant,
                EffectApplication::Status(Status::Stunned, 1),
            ),
        ),
    }];

    pub const ANCIENT_CONSTRUCT: [Spell; 2] = [
        Spell {
            name: "Beam",
            glyph: (Glyph::Fire, 2),
            style: (Style::Void, 1),
            ability: Ability::duo(
                PriorityType::LowHealth,
                TargetType::Enemy(2),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
                Effect::new(
                    1,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Stunned, 1),
                ),
            ),
        },
        Spell {
            name: "Smash",
            glyph: (Glyph::Earth, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(3),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
    ];

    pub const ANGEL: [Spell; 2] = [
        Spell {
            name: "Smite",
            glyph: (Glyph::Fire, 2),
            style: (Style::Void, 1),
            ability: Ability::duo(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
                Effect::new(
                    1,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Stunned, 1),
                ),
            ),
        },
        Spell {
            name: "Heal",
            glyph: (Glyph::Water, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Ally(1),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Heal),
            ),
        },
    ];

    pub const ARCHON: [Spell; 2] = [
        Spell {
            name: "Fire ball",
            glyph: (Glyph::Fire, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Ally(3),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Water spiral",
            glyph: (Glyph::Water, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(3),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
    ];

    pub const TEMPLAR: [Spell; 2] = [
        Spell {
            name: "Fire ball",
            glyph: (Glyph::Fire, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Ally(3),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Water spiral",
            glyph: (Glyph::Water, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Enemy(3),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
    ];

    pub const ELEMENTAL: [Spell; 4] = [
        spells::fire::FIREBALL,
        spells::water::SUBMERGE,
        spells::earth::EARTHQUAKE,
        spells::air::LIGHTNING,
    ];

    pub const GUARDIAN: [Spell; 2] = [
        Spell {
            name: "Smash",
            glyph: (Glyph::Earth, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(3),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Shield",
            glyph: (Glyph::Earth, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Ally(1),
                Effect::new(
                    5,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Barrier(Glyph::Earth), 1),
                ),
            ),
        },
    ];

    pub const RAT: [Spell; 1] = [Spell {
        name: "Bite",
        glyph: (Glyph::Air, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::Squishy,
            TargetType::Enemy(1),
            Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const SLIME: [Spell; 1] = [Spell {
        name: "Slap",
        glyph: (Glyph::Water, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::Squishy,
            TargetType::Enemy(1),
            Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const VOIDLING: [Spell; 1] = [Spell {
        name: "Zap",
        glyph: (Glyph::Void, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::Squishy,
            TargetType::Enemy(1),
            Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const VOID_SPAWN: [Spell; 2] = [
        Spell {
            name: "Spark",
            glyph: (Glyph::Void, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(2),
                Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Zap",
            glyph: (Glyph::Void, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(1),
                Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
    ];

    pub const VOID_WALKER: [Spell; 2] = [
        Spell {
            name: "Zap",
            glyph: (Glyph::Void, 1),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(1),
                Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Void Blast",
            glyph: (Glyph::Void, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(3),
                Effect::new(
                    3,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Weakened, 1),
                ),
            ),
        },
    ];

    pub const VOID_LORD: [Spell; 0] = [];

    pub const FALLEN_ANGEL: [Spell; 2] = [
        Spell {
            name: "Smite",
            glyph: (Glyph::Fire, 2),
            style: (Style::Void, 1),
            ability: Ability::duo(
                PriorityType::LowHealth,
                TargetType::Enemy(1),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
                Effect::new(
                    1,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Stunned, 1),
                ),
            ),
        },
        Spell {
            name: "Heal",
            glyph: (Glyph::Water, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Ally(1),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Heal),
            ),
        },
    ];

    pub const HUMAN: [Spell; 1] = [Spell {
        name: "Stab",
        glyph: (Glyph::Air, 1),
        style: (Style::Void, 1),
        ability: Ability::single(
            PriorityType::Squishy,
            TargetType::Enemy(1),
            Effect::new(3, EffectDuration::Instant, EffectApplication::Damage),
        ),
    }];

    pub const CONSTRUCT: [Spell; 2] = [
        Spell {
            name: "Smash",
            glyph: (Glyph::Earth, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::Squishy,
                TargetType::Enemy(3),
                Effect::new(5, EffectDuration::Instant, EffectApplication::Damage),
            ),
        },
        Spell {
            name: "Shield",
            glyph: (Glyph::Earth, 2),
            style: (Style::Void, 1),
            ability: Ability::single(
                PriorityType::LowHealth,
                TargetType::Ally(1),
                Effect::new(
                    5,
                    EffectDuration::Instant,
                    EffectApplication::Status(Status::Barrier(Glyph::Earth), 1),
                ),
            ),
        },
    ];
}

#[derive(Clone, Debug)]
pub struct Monster {
    pub id: Option<Index>,
    pub name: String,
    pub monster_type: MonsterType,
    pub affinity: Affinity,
    pub acceptance: Acceptance,
    pub hp: u32,
    pub max_hp: u32,
    pub status: StatusSet,
}

impl Monster {
    pub fn new(
        name: &str,
        monster_type: &MonsterType,
        difficulty: u8,
    ) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            monster_type: monster_type.clone(),
            affinity: Affinity::new(),
            acceptance: Acceptance::new(),
            hp: monster_type.hp(difficulty as u32),
            max_hp: monster_type.hp(difficulty as u32),
            status: StatusSet::new(),
        }
    }

    pub fn get_abilities(&self) -> Vec<Spell> {
        MonsterType::ability_with_style(self.monster_type.clone(), self.acceptance.get_highest())
    }

    pub fn augment(&self, glyph: &Glyph) -> u16 {
        self.affinity.val(glyph) as u16
    }

    pub fn resist(&self, glyph: &Glyph) -> u16 {
        self.affinity.val(glyph) as u16
    }
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Monster {}