use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum BiomeType {
    None,
    Water,
    Plains,
    Hills,
    Mountains,
    Marsh,
    Plateau,
    Highlands,
    Coastal,
    SaltMarsh,
}

impl BiomeType {
    pub fn iter() -> Iter<'static, BiomeType> {
        static BTYPES: [BiomeType; 10] = [
            BiomeType::None,
            BiomeType::Water,
            BiomeType::Plains,
            BiomeType::Hills,
            BiomeType::Mountains,
            BiomeType::Marsh,
            BiomeType::Plateau,
            BiomeType::Highlands,
            BiomeType::Coastal,
            BiomeType::SaltMarsh,
        ];
        BTYPES.iter()
    }
}
