use {
    serde::{Serialize, Deserialize},
    serde_json::{Result, Value}
};

/* base function imports */

/// Parses a `.json` data file
/// into a ute2-usable `TileOne`
fn parse_tile(tile_path: &str) -> TileOne {
    unimplemented!()
}

/// parses a Vec of parsed `TileOne`s into a
/// ute2-usable `TileArray`
fn parse_map(tile_vec: Vec<TileOne>) -> TileArray {
    unimplemented!()
}

/* define tile and map types */
struct TileOne {
    lonely_data: Value,
}

struct TileArray {
    map_data: Vec<TileOne>
}

/* conditionally import submodules */
#[cfg(feature="generate")]
pub mod generate;