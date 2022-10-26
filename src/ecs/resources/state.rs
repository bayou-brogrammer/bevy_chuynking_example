#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Assets,
    Loading,

    MainMenu,
    WorldGen,
    Embark,
    RegionGen,
    InGame,

    // Debug States
    PlanetGen,
    PlanetGenWait,
    GenRegion,
    RegionGenWait,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnState {
    AwaitingInput,
    Ticking,
    Dead,
}
