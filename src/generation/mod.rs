use rand::{Rng, thread_rng};

use crate::datastructures::{Tile, TileContent, Base, EatableStruct};
use crate::constants::*;

pub fn setup_tiles() -> Vec<Vec<Tile>> {
    let mut map:Vec<Vec<Tile>> = Vec::new();
    for y in 0..HEIGHT {
        let mut row:Vec<Tile> = Vec::new();
        for x in 0..WIDTH {
            row.push(Tile {
                x:x,
                y:y,
                content: set_tile_content((x,y)),
            });
        }
        map.push(row);
    }

    map
}


fn set_tile_content(coords:(i32,i32)) -> Option<TileContent> {
    if thread_rng().gen::<f64>() <= FOOD_CHANCE {
        return Some(TileContent::Eatable(EatableStruct { 
            base: Base {
                icon:FOOD_ICON.to_string(),
                x:coords.0,
                y:coords.1
            }, 
            eatable:true,
            value: (thread_rng().gen_range(1..MAX_FOOD_REJUV)) 
        }));
    } 
    else {
        return Some(TileContent::Empty(Base { 
            icon:EMPTY_ICON.to_string(),
            x:coords.0,
            y:coords.1
        }));
    };

   
}