use std::collections::HashMap;
use std::io::{Write, Read, Cursor};
use std::fs::File;

use generational_arena::{Arena, Index};

use crate::a::q::battle::Battle;

pub mod e;

use e::mon::Monster;
use e::party::Party;
use e::wiz::Wizard;

mod inp;
mod out;

use inp::Inputable;

const ACTIVE_FILENAME: &str = "./assets/active.colosseum";
const DEAD_FILENAME: &str = "./assets/dead.colosseum";

pub struct Colosseum {
    wizards: Arena<Wizard>,
    battles: Arena<Battle>,
    parties: Arena<Party>,
    monsters: Arena<Monster>,
}

macro_rules! write_arena {
    ($col:ident . $as:ident { $($to:ident),* $(,)? } << $b:ident) => {{
        $b.write(&$col.$as.len().to_le_bytes())?;
        for (_, it) in $col.$as.iter() {
            let line = it.as_bytes();
            match line {
                Ok(line) => {
                    $b.write(&line.len().to_le_bytes()).expect(format!("Failed to write {:?} size", stringify!($as)).as_str());
                    $b.write(&line)?;
                    $($col.write::<$to, _>(it, &mut $b);)*
                }
                Err(e) => {$b.write_fmt(format_args!("\n{:?}: {}\n", stringify!($as), e))?;}
            }
        }
    }}
}

macro_rules! read_arena {
    ($col:ident . $as:ident : $tas:ident { $($map:ident => $to:ident),* $(,)? } >> $f:ident) => {{
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        let mut map = HashMap::new();
        let mut $as = Arena::new();
        $f.read_exact(&mut usize_buf).expect(format!("Failed to read number of {}", stringify!($as)).as_str());
        let number_of = usize::from_le_bytes(usize_buf);
        for _ in 0..number_of {
            $f.read_exact(&mut usize_buf).expect(format!("Failed to read {} size", stringify!($as)).as_str());
            let next_size = usize::from_le_bytes(usize_buf);
            let mut buf = vec![0u8; next_size];
            $f.read_exact(&mut buf).expect(format!("Failed to read {}", stringify!($as)).as_str());
            let it = $tas::from_bytes(&mut Cursor::new(&buf)).expect(format!("Failed to load {}", stringify!($as)).as_str());
            let id = $as.insert(it);
            $as[id].id = Some(id);
            map.insert(id.into_raw_parts().0, id);

            $($col.read::<$to, _>($as.get(id).unwrap(), &mut $f, &$map);)*
        }
        $col.$as = $as;
        map
    }}
}

impl Colosseum {
    pub fn new() -> std::io::Result<Colosseum> {
        let mut col = Colosseum {
            wizards: Arena::new(),
            battles: Arena::new(),
            parties: Arena::new(),
            monsters: Arena::new(),
        };
        col.load()?;
        Ok(col)
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        if !std::path::Path::new(ACTIVE_FILENAME).exists() {
            return Ok(());
        }
        // Compute battles
        let mut f = File::open(ACTIVE_FILENAME).expect(format!("Failed to open {}", ACTIVE_FILENAME).as_str());

        let battle_map = read_arena!(self.battles: Battle {} >> f);
        let party_map = read_arena!(self.parties: Party  {} >> f);

        read_arena!(self.wizards:  Wizard  {battle_map => Battle, party_map => Party} >> f);
        read_arena!(self.monsters: Monster {battle_map => Battle                    } >> f);
        f.flush()
    }

    pub fn save(&self) -> std::io::Result<()> {
        use out::Outputable;
        let mut f = File::create(ACTIVE_FILENAME)?;
        write_arena!(self.battles  {}              << f);        
        write_arena!(self.parties  {}              << f);        
        write_arena!(self.wizards  {Battle, Party} << f);
        write_arena!(self.monsters {Battle}        << f);
        f.flush()
    }
}

pub trait Idable {
    fn id(&self) -> Option<Index>;
    fn set_id(&mut self, id: Index);
}
impl Idable for Wizard {
    fn id(&self) -> Option<Index> {
        self.id
    }
    fn set_id(&mut self, id: Index) {
        self.id = Some(id);
    }
}
impl Idable for Monster {
    fn id(&self) -> Option<Index> {
        self.id
    }
    fn set_id(&mut self, id: Index) {
        self.id = Some(id);
    }
}
impl Idable for Party {
    fn id(&self) -> Option<Index> {
        self.id
    }
    fn set_id(&mut self, id: Index) {
        self.id = Some(id);
    }
}
impl Idable for Battle {
    fn id(&self) -> Option<Index> {
        self.id
    }
    fn set_id(&mut self, id: Index) {
        self.id = Some(id);
    }
}
pub trait ColosseumArena<T> where T : Idable {
    fn get_arena(&self) -> &Arena<T>;
    fn get_arena_mut(&mut self) -> &mut Arena<T>;

    fn get(&self, id: Index) -> &T {
        self.get_arena().get(id).unwrap()
    }
    fn get_mut(&mut self, id: Index) -> &mut T {
        self.get_arena_mut().get_mut(id).unwrap()
    }
    fn insert(&mut self, t: T) -> Index {
        let id = self.get_arena_mut().insert(t);
        self.get_arena_mut().get_mut(id).unwrap().set_id(id);
        id
    }
    fn remove(&mut self, id: Index) -> Option<T> {
        self.get_arena_mut().remove(id)
    }
    fn iter(&self) -> generational_arena::Iter<T> {
        self.get_arena().iter()
    }
}

impl ColosseumArena<Wizard> for Colosseum {
    fn get_arena(&self) -> &Arena<Wizard> {
        &self.wizards
    }
    fn get_arena_mut(&mut self) -> &mut Arena<Wizard> {
        &mut self.wizards
    }
}
impl ColosseumArena<Battle> for Colosseum {
    fn get_arena(&self) -> &Arena<Battle> {
        &self.battles
    }
    fn get_arena_mut(&mut self) -> &mut Arena<Battle> {
        &mut self.battles
    }
}
impl ColosseumArena<Party> for Colosseum {
    fn get_arena(&self) -> &Arena<Party> {
        &self.parties
    }
    fn get_arena_mut(&mut self) -> &mut Arena<Party> {
        &mut self.parties
    }
}
impl ColosseumArena<Monster> for Colosseum {
    fn get_arena(&self) -> &Arena<Monster> {
        &self.monsters
    }
    fn get_arena_mut(&mut self) -> &mut Arena<Monster> {
        &mut self.monsters
    }
}

trait Container<B> {
    fn position(&self, id: Index) -> Option<usize>;
    fn put(&mut self, id: Index, position: usize);
}

trait Association<A> where A: Idable {
    fn write<B, T>(&self, a: &A, f: &mut T) where B: Idable + Container<A>, Self: ColosseumArena<B>, T: Write {
        let output : Vec<(Index,Option<usize>)> = self.iter().map(|(id, b)| (id, b.position(a.id().unwrap()))).filter(|(_, p)| p.is_some()).collect();
        f.write(&output.len().to_le_bytes()).expect("Failed to write number of battles");
        for (id, ui) in output {
            f.write(&id.into_raw_parts().0.to_le_bytes()).expect("Failed to write index id");
            f.write(&ui.unwrap().to_le_bytes()).expect("Failed to write usize id");
        }
    }

    fn read<B, T>(&mut self, a: &A, f: &mut T, map: &HashMap<usize, Index>) -> std::io::Result<()> where  B: Idable + Container<A>, Self: ColosseumArena<B>, T: Read {
        let mut usize_buf = [0u8; std::mem::size_of::<usize>()];
        f.read_exact(&mut usize_buf)?;
        let number_of_battles = usize::from_le_bytes(usize_buf);
        for _ in 0..number_of_battles {
            f.read_exact(&mut usize_buf).expect("Failed to read id index");
            let index = usize::from_le_bytes(usize_buf);
            let id = map.get(&index).unwrap();
            f.read_exact(&mut usize_buf).expect("Failed to read index");
            let index = usize::from_le_bytes(usize_buf);
            self.get_arena_mut().get_mut(*id).unwrap().put(a.id().unwrap(), index);
        }
        Ok(())
    }
}

impl Container<Wizard> for Battle {
    fn position(&self, id: Index) -> Option<usize> {
        self.allies.iter().position(|i| *i == id)
    }
    fn put(&mut self, id: Index, position: usize) {
        while position >= self.allies.len() {
            self.allies.push(id);
        }
        self.allies[position] = id;
    }
}
impl Container<Wizard> for Party {
    fn position(&self, id: Index) -> Option<usize> {
        self.members.iter().position(|i| *i == id)
    }
    fn put(&mut self, id: Index, position: usize) {
        while position >= self.members.len() {
            self.members.push(id);
        }
        self.members[position] = id;
    }
}
impl Container<Monster> for Battle {
    fn position(&self, id: Index) -> Option<usize> {
        self.allies.iter().position(|i| *i == id)
    }
    fn put(&mut self, id: Index, position: usize) {
        while position >= self.allies.len() {
            self.allies.push(id);
        }
        self.allies[position] = id;
    }
}

impl Association<Wizard> for Colosseum {}
impl Association<Monster> for Colosseum {}