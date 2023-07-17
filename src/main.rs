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
    let (mut id_map,mut data_map) = generation::setup_tiles();
    
    // init a new "player"
    let entity = Entity(EntityStruct {
        base:Base {
            icon:ENTITY_ICON.to_string(),
        },
        hunger: MAX_HUNGER,
        health:HEALTH,
        state: State::Idle
    });
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

    officejet::clear();
    officejet::print(id_map.clone(), data_map.clone());

    let mut start_time = Instant::now();    // capture start time

    loop {
        // put tick-dependant code here
        if start_time.elapsed() >= Duration::from_millis(MAX_TICK) {
            start_time = Instant::now();
            
            
            // refresh the display

        }

        // and the rest out here
    }


}

/*
refurb

- 2d vec -> map, used for printing
- list with food nodes
- list with entities
- generate id's for each of em

*/