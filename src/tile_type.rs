use enumflags2::bitflags;
use serde::{Deserialize, Serialize};

// #[bitflags]
#[repr(u16)]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TileType {
    #[default]
    Floor,
    Wall,
    Water,
    Sand,
    Soil,
    Tree(TreeType),
    Plant(PlantType),
}

#[bitflags]
#[repr(u8)]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum PlantType {
    #[default]
    Grass,
    Daisy,
    Heather,
}

#[bitflags]
#[repr(u8)]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TreeType {
    #[default]
    Evergreen,
    Deciduous,
}
