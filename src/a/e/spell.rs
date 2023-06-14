use super::{Glyph, Style};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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
    pub fn as_output(&self) -> u8 {
        match self {
            PriorityType::Squishy => 0,
            PriorityType::Tanky => 1,
            PriorityType::LowHealth => 2,
            PriorityType::HighHealth => 3,
            PriorityType::HasStatus(status) => 4 + status.as_output(),
            PriorityType::NoStatus(status) => 18 + status.as_output(),
        }
    }
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
    pub fn as_output(&self) -> Vec<u8> {
        match self {
            PriorityTypes::Single(priority) => vec![0, priority.as_output(), 0],
            PriorityTypes::Or(priority1, priority2) => vec![1, priority1.as_output(), priority2.as_output()],
            PriorityTypes::And(priority1, priority2) => vec![2, priority1.as_output(), priority2.as_output()],
        }
    }
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
    pub fn as_output(&self) -> Vec<u8> {
        match self {
            TargetType::MeAlone => vec![0,0],
            TargetType::Ally(index) => vec![1,*index],
            TargetType::Enemy(index) => vec![2,*index],
        }
    }
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

    pub const fn len(&self) -> u8 {
        match &self.effects {
            EffectProgression::None => 0,
            EffectProgression::Single(_) => 1,
            EffectProgression::Duo(_, _) => 2,
            EffectProgression::Trio(_, _, _) => 3,
        }
    }

    pub fn as_output(&self) -> std::io::Result<Vec<u8>> {
        let mut output = Vec::new();
        output.extend(self.priority.as_output());
        output.extend(self.target.as_output());
        output.extend(self.effects.as_output()?);
        Ok(output)
    }

    pub fn from_buf(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        use byteorder::ReadBytesExt;
        let priority = PriorityTypes::from_3u8(buf.read_u8()?, buf.read_u8()?, buf.read_u8()?)?;
        let target = TargetType::from_2u8(buf.read_u8()?, buf.read_u8()?)?;
        let effects = EffectProgression::from_buf(buf)?;
        Ok(Self {
            priority,
            target,
            effects,
        })
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

    pub fn as_output(&self) -> std::io::Result<Vec<u8>> {
        use byteorder::{LittleEndian, WriteBytesExt};
        let mut output = Vec::new();
        output.write_u8(self.glyph.0.as_u8())?;
        output.write_u16::<LittleEndian>(self.glyph.1)?;
        output.write_u8(self.style.0.as_u8())?;
        output.write_u16::<LittleEndian>(self.style.1)?;

        let spells = spells::BY_GLYPH_AND_STYLE
            .get(&(self.glyph.0))
            .unwrap()
            .get(&(self.style.0))
            .unwrap();
        let spell_index = spells.iter().position(|spell| spell.name == self.name).unwrap();
        output.extend_from_slice(&spell_index.to_le_bytes());
        output.extend(self.ability.as_output()?);
        Ok(output)
    }

    pub fn from_buf(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::Read;
        let glyph = Glyph::from_u8(buf.read_u8()?);
        let glyph_value = buf.read_u16::<LittleEndian>()?;
        let style = Style::from_u8(buf.read_u8()?);
        let style_value = buf.read_u16::<LittleEndian>()?;

        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        buf.read_exact(&mut usize_buf)?;
        let spell_index = usize::from_le_bytes(usize_buf);

        Ok(Self {
            name: spells::BY_GLYPH_AND_STYLE
                .get(&glyph)
                .unwrap()
                .get(&style)
                .unwrap()
                .get(spell_index)
                .unwrap()
                .name,
            glyph: (glyph, glyph_value),
            style: (style, style_value),
            ability: Ability::from_buf(buf)?,
        })
    }
}

impl EffectProgression {
    pub fn as_output(&self) -> std::io::Result<Vec<u8>> {
        use byteorder::WriteBytesExt;
        let mut output = Vec::new();
        match self {
            EffectProgression::None => {
                output.write_u8(0)?;
            }
            EffectProgression::Single(effect) => {
                output.write_u8(1)?;
                output.extend(effect.as_output()?);
            }
            EffectProgression::Duo(effect1, effect2) => {
                output.write_u8(2)?;
                output.extend(effect1.as_output()?);
                output.extend(effect2.as_output()?);
            }
            EffectProgression::Trio(effect1, effect2, effect3) => {
                output.write_u8(3)?;
                output.extend(effect1.as_output()?);
                output.extend(effect2.as_output()?);
                output.extend(effect3.as_output()?);
            }
        }
        Ok(output)
    }
    pub fn from_buf(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        use byteorder::ReadBytesExt;
        let kind = buf.read_u8()?;
        match kind {
            0 => Ok(Self::None),
            1 => Ok(Self::Single(Effect::from_cursor(buf)?)),
            2 => Ok(Self::Duo(Effect::from_cursor(buf)?, Effect::from_cursor(buf)?)),
            3 => Ok(Self::Trio(
                Effect::from_cursor(buf)?,
                Effect::from_cursor(buf)?,
                Effect::from_cursor(buf)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid effect progression kind",
            )),
        }
    }
}

impl Effect {
    pub fn as_output(&self) -> std::io::Result<Vec<u8>> {
        use byteorder::{LittleEndian, WriteBytesExt};
        let mut output = Vec::new();
        output.write_u16::<LittleEndian>(self.value)?;
        output.extend(self.duration.as_output()?);
        output.extend(self.application.as_output()?);
        Ok(output)
    }
    pub fn from_cursor(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        use byteorder::ReadBytesExt;
        let value = buf.read_u16::<byteorder::LittleEndian>()?;
        let duration = EffectDuration::from_cursor(buf)?;
        let application = EffectApplication::from_cursor(buf)?;
        Ok(Self {
            value,
            duration,
            application,
        })
    }
}

impl EffectDuration {
    pub fn as_output(&self) -> std::io::Result<Vec<u8>> {
        use byteorder::{LittleEndian, WriteBytesExt};
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

    pub fn from_cursor(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        use byteorder::ReadBytesExt;
        let kind = buf.read_u8()?;
        match kind {
            0 => Ok(Self::OverTime(buf.read_u16::<byteorder::LittleEndian>()?)),
            1 => Ok(Self::Growth(
                buf.read_u16::<byteorder::LittleEndian>()?,
                buf.read_u16::<byteorder::LittleEndian>()?,
            )),
            2 => Ok(Self::AfterXTime(buf.read_u16::<byteorder::LittleEndian>()?)),
            3 => Ok(Self::Instant),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid effect duration kind",
            )),
        }
    }
}

impl EffectApplication {
    pub fn as_output(&self) -> std::io::Result<Vec<u8>> {
        use byteorder::{LittleEndian, WriteBytesExt};
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
                output.write_u8(status.as_output())?;
                output.write_u16::<LittleEndian>(*duration)?;
            },
            EffectApplication::RemoveStatus(status) => {
                output.write_u8(3)?;
                output.write_u8(status.as_output())?;
            },
        }
        Ok(output)
    }

    pub fn from_cursor(buf: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        use byteorder::ReadBytesExt;
        let kind = buf.read_u8()?;
        match kind {
            0 => Ok(Self::Damage),
            1 => Ok(Self::Heal),
            2 => Ok(Self::Status(
                Status::from_u8(buf.read_u8()?),
                buf.read_u16::<byteorder::LittleEndian>()?,
            )),
            3 => Ok(Self::RemoveStatus(Status::from_u8(buf.read_u8()?))),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid effect application kind",
            )),
        }
    }
}

impl Status {
    pub fn as_output(&self) -> u8 {
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
    pub fn from_u8(id: u8) -> Status {
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
