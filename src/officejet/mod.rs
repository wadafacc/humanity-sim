use crate::datastructures::*;

/// The pinnacle of human engineering: The Printer. A feat of mechanics, technology and analog media that engineers to this day can't fix.
pub fn print(mut map:Vec<Vec<Tile>>) {
    let mut entity:EntityStruct = EntityStruct {..Default::default()};
    for row in map {
        for tile in row {
            match tile.content.unwrap() {
                TileContent::Empty(e) => print!("| {} ", e.icon),
                TileContent::Eatable(e) => print!("| {} ", e.base.icon),
                TileContent::Entity(e) => {entity = e;print!("| {} ", entity.base.icon)}
            }
        }
        println!("|");
    }
    // print details of the entity
    println!("\x1b[38;5;214mHUNGER {}\x1b[0m <||> \x1b[38;5;1mHEALTH {}\x1b[0m", entity.hunger, entity.health);
}

/// clear the screen
pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}