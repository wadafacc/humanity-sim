#[derive(Clone)]
pub struct Tile {
    pub x:i32,
    pub y:i32,
    pub content: Option<TileContent>,
}
#[derive(Default, Clone)]
pub struct Base {
    pub icon: String,
    pub x:i32,
    pub y:i32

}

#[derive(Default, Clone)]
pub struct EntityStruct {
    pub base:Base,
    pub hunger: i32,
    pub health: i32,
    pub state: State
}

#[derive(Clone)]
pub struct EatableStruct {
    pub base:Base,
    pub value:i32,
    pub eatable:bool
    // other properties can be added here
}

#[derive(Clone)]
pub enum TileContent {
    Empty(Base),
    Entity(EntityStruct),
    Eatable(EatableStruct),
    // mineable()
}

#[derive(Default, Clone)]
pub enum State {
    #[default] Idle,
    Searching,
    Eating,
    Walking
}