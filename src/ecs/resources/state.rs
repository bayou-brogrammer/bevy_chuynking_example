#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Assets,
    Loading,

    MainMenu,
    PlanetGen,
    Embark,
    RegionGen,
    InGame,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnState {
    AwaitingInput,
    Ticking,
    Dead,
}
