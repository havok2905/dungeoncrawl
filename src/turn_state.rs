#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    MonsterTurn,
    PlayerTurn,
    GameOver,
    Victory,
    NextLevel,
}
