use generational_arena::Index;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Party {
    pub uuid: Uuid,
    pub id: Option<Index>,
    pub members: Vec<Index>,
}

impl Party {
    pub fn new(members: Vec<Index>) -> Party {
        Party {
            id: None,
            uuid: Uuid::new_v4(),
            members
        }
    }

    
}