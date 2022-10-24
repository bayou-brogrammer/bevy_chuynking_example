use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiverStep {
    pub pos: IVec2,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct River {
    pub name: String,
    pub start: IVec2,
    pub steps: Vec<RiverStep>,
}

impl River {
    pub fn new() -> Self {
        Self { name: String::new(), start: IVec2::ZERO, steps: Vec::new() }
    }
}
