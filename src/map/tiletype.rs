use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,         //Opaque
    Stalagmite,   //Opaque
    Stalactite,   //Opaque
    Floor,        //Walkable
    DownStairs,   //Walkable
    UpStairs,     //Walkable
    Road,         //Walkable
    Grass,        //Walkable
    ShallowWater, //Walkable
    DeepWater,    //Not Walkable
    WoodFloor,    //Walkable
    Bridge,       //Walkable
    Gravel,       //Walkable
}

pub fn tile_walkable(tt: TileType) -> bool {
    match tt {
        TileType::Floor
        | TileType::DownStairs
        | TileType::UpStairs
        | TileType::Road
        | TileType::Grass
        | TileType::ShallowWater
        | TileType::WoodFloor
        | TileType::Bridge
        | TileType::Gravel => true,
        _ => false,
    }
}

pub fn tile_opaque(tt: TileType) -> bool {
    match tt {
        TileType::Wall | TileType::Stalagmite | TileType::Stalactite => true,
        _ => false,
    }
}

pub fn tile_cost(tt: TileType) -> f32 {
    match tt {
        TileType::Road => 0.8,
        TileType::Grass => 1.1,
        TileType::ShallowWater => 1.2,
        _ => 1.0,
    }
}
