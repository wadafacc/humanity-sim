/*
----- CONSTANTS FOR CONFIGURATION -----
*/

// tick system config
pub const MAX_TICK:u64 = 1000; // Milliseconds per Tick
// map config
pub const HEIGHT:i32 = 16;
pub const WIDTH:i32 = 16;

// entity config
pub const MAX_HUNGER: i32 = 20;
pub const HEALTH: i32 = 100;
pub const HUNGER_DECAY: i32 = 2;  // decay per Tick -> Hunger
pub const ENTITY_ICON: &str = "\x1b[38;5;196m◈\x1b[0m";

// food config
pub const FOOD_CHANCE: f64 = 0.05;
pub const MAX_FOOD_REJUV: i32 = 50;   // hunger points restored by a food node
pub const EMPTY_ICON: &str = " ";   // used if the field is not a food tile

pub const FOOD_ICON: &str = "\x1b[38;5;120m⧄\x1b[0m";
/*
- The first half ("\x1b[38;5;120m") sets the color for the upcoming text.
- The text (in this case "⧄") is then colored in that color
- The last part ("\x1b[0m") sets the color back to black for the text after the colored symbol
https://talyian.github.io/ansicolors/ For reference
*/
