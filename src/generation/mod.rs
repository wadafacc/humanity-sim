use uuid::{Uuid};
use std::collections::HashMap;

use rand::{Rng, thread_rng};

use crate::datastructures::{Tile, TileContent, Base, EatableStruct};
use crate::constants::*;


pub fn init_map(entity: TileContent) -> (Vec<Vec<Uuid>>, HashMap<Uuid, TileContent>) {
    let (mut id_map,mut data_map) = setup_tiles();
    if let TileContent::Entity(e) = entity.clone() {
        println!("{:?}",e);
        id_map[0][0] = e.id;
        data_map.insert(e.id, entity);
    };
    (id_map, data_map)
}


pub fn setup_tiles() -> (Vec<Vec<Uuid>>, HashMap<Uuid, TileContent>) {
    let mut map:Vec<Vec<Uuid>> = Vec::new();
    let mut hashmap:HashMap<Uuid, TileContent> = HashMap::new();
    for y in 0..HEIGHT {
        let mut row:Vec<Uuid> = Vec::new();
        for x in 0..WIDTH {
            let tile = Tile {
                id: Uuid::new_v4(),
                x:x,
                y:y,
                content: set_tile_content(),
            };
            row.push(tile.id);
            hashmap.insert(tile.id,tile.content);

        }
        map.push(row);
    }

    (map, hashmap)
}

fn set_tile_content() -> TileContent {
    if thread_rng().gen::<f64>() <= FOOD_CHANCE {
        return TileContent::Eatable(EatableStruct { 
            base: Base {
                icon:FOOD_ICON.to_string(),
            }, 
            eatable:true,
            value: (thread_rng().gen_range(1..MAX_FOOD_REJUV)) 
        });
    } 
    else {
        return TileContent::Empty(Base { 
            icon:EMPTY_ICON.to_string(),
        });
    };

   
}