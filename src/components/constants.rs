use std::ops::Range;

pub const TILE_SIZE: f32 = 32.0;
pub const MAP_TILES: Range<i32> = -NUMBER_OF_TILES..NUMBER_OF_TILES;
pub const MAP_SIZE: f32 = TILE_SIZE * NUMBER_OF_TILES as f32;
const NUMBER_OF_TILES: i32 = 64;
