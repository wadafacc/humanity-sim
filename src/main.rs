use std::time::{Instant, Duration};

use constants::*;
use datastructures::{TileContent, Tile};

use crate::{constants::{MAX_TICK,ENTITY_ICON}, datastructures::{State, TileContent::Entity, EntityStruct, Base}};

mod constants;
mod generation;
mod datastructures;
mod officejet;

fn main() {
    // generate a map for the current runthrough
    let mut map = generation::setup_tiles();
    
    // init a new "player"
    let entity = Entity(EntityStruct {
        base:Base {
            x:0,
            y:0,
            icon:ENTITY_ICON.to_string(),
        },
        hunger: MAX_HUNGER,
        health:HEALTH,
        state: State::Idle
    });
    map[0][0].content = Some(entity.to_owned());
    /*
    TODO:
    - Setup Tick System [DONE]
    - Setup Tile System [DONE]
    - Setup Display System [DONE]

    Gen:
    - Spawn Food Tiles [DONE]

    "AI":
    - Roam
    - Get food if hungry
    */

    let mut start_time = Instant::now();    // capture start time
    let mut entity = &mut EntityStruct {..Default::default()};
    match map[0][0].content.as_mut().unwrap() {
        TileContent::Entity(e) => entity = e,
        TileContent::Eatable(e) => (),
        _ => (),
    };
    // main loop
    loop {
        // put tick-dependant code here
        if start_time.elapsed() >= Duration::from_millis(MAX_TICK) {
            start_time = Instant::now();
            


            entity.hunger -= HUNGER_DECAY;
            
            // refresh the display
            officejet::clear();
            officejet::print(main_map.clone());
        }

        // and the rest out here
    }


}

/*
refurb


*/