use uuid::Uuid;

#[derive(Clone,Debug)]
pub struct Tile {
    pub id: Uuid,
    pub x:i32,
    pub y:i32,
    pub content: TileContent,
}
#[derive(Clone,Debug, Default)]
pub struct Base {
    pub icon: String,
}

#[derive(Clone,Debug, Default)]
pub struct EntityStruct {
    pub id: Uuid,
    pub base:Base,
    pub hunger: i32,
    pub health: i32,
    pub state: State
}

#[derive(Clone,Debug, Default)]
pub struct EatableStruct {
    pub base:Base,
    pub value:i32,
    pub eatable:bool
    // other properties can be added here
}

#[derive(Clone,Debug)]
pub enum TileContent {
    Empty(Base),
    Entity(EntityStruct),
    Eatable(EatableStruct),
    // mineable()
}

#[derive(Default, Clone,Debug)]
pub enum State {
    #[default] Idle,
    Searching,
    Eating,
    Walking
}