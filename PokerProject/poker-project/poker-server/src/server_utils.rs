/// contains utilities to be used across server portions


/// Server states a player can be in
#[derive(Debug, PartialEq)]
pub enum ServerState {
    Connection,
    Lobby,
    InGame,
}
